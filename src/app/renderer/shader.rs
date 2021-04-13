use std::ptr;
use gl::types::*;

use super::*;

// TODO move compiling, link
// TODO return Option/Result?
pub(super) fn compile_shader(src: &str, shader_type: GLenum) -> Shader {
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);

        // Try to compile
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check compilation status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

pub(super) fn compile_shader_from_file<P: AsRef<Path>>(path: P, shader_type: GLenum) -> Shader {
    let buffer = std::fs::read_to_string(path)
        //.expect(&format!("File {} not found", path.display()));
        .expect("File not found");

    compile_shader(&buffer, shader_type)
}

pub(super) fn link_shader_program(vs: Shader, fs: Shader) -> Program {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Get link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}

pub(super) fn create_shader_program<P: AsRef<Path>>(vs_path: P, fs_path: P) -> Program {
    let vs = compile_shader_from_file(vs_path, gl::VERTEX_SHADER);
    let fs = compile_shader_from_file(fs_path, gl::FRAGMENT_SHADER);
    let program = link_shader_program(vs, fs);
    program
}

