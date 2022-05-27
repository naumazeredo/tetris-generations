use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use gl::types::*;

use super::*;
use crate::linalg::Mat4;

const MAX_NAME_LENGTH: usize = 256;

// It's very annoying that *type* is a keyword in Rust...
#[derive(Copy, Clone, Debug, ImDraw)]
pub enum UniformVariant {
    Float,
    Float2,
    Float3,
    Float4,
    Mat4,
    Texture2D,
    //Sampler2D,
}

#[derive(PartialEq, Clone, Debug, ImDraw)]
pub enum UniformData {
    Float(f32),
    Float2([f32; 2]),
    Float3([f32; 3]),
    Float4([f32; 4]),
    Mat4(Mat4),
    Texture2D(TextureRef),
    Texture2DNotSet,
}

impl std::convert::From<UniformVariant> for UniformData {
    fn from(variant: UniformVariant) -> Self {
        match variant {
            UniformVariant::Float     => UniformData::Float (0.0),
            UniformVariant::Float2    => UniformData::Float2([0.0; 2]),
            UniformVariant::Float3    => UniformData::Float3([0.0; 3]),
            UniformVariant::Float4    => UniformData::Float4([0.0; 4]),
            UniformVariant::Mat4      => UniformData::Mat4  (Mat4::new()),
            UniformVariant::Texture2D => UniformData::Texture2DNotSet,
        }
    }

}

#[derive(Debug, ImDraw)]
pub struct UniformInfo {
    pub(super) name:     String,
    pub(super) variant:  UniformVariant,
    pub(super) location: GLint,
}

#[derive(Debug, ImDraw)]
pub struct Shader {
    pub(super) id: GLuint,
    pub(super) uniforms: Vec<UniformInfo>,
}

pub type ShaderRef = Rc<RefCell<Shader>>;

impl Shader {
    pub fn new<P: AsRef<Path>>(vs: P, fs: P) -> ShaderRef {
        create_shader_program(vs, fs)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// TODO return Option (and rename to try_)
fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    let shader;

    unsafe {
        shader = gl::CreateShader(shader_type);

        // Try to compile
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
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
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }

    shader
}

// @TODO return Option (and rename to try_)
fn compile_shader_from_file<P: AsRef<Path>>(path: P, shader_type: GLenum) -> GLuint {
    // @TODO logging
    println!("Compiling shader: {}", path.as_ref().display());

    let buffer = std::fs::read_to_string(path)
        //.expect(&format!("File {} not found", path.display())); // @XXX move occurs above
        .expect("File not found");

    compile_shader(&buffer, shader_type)
}

// @TODO return Option/Result (and rename to try_)
fn link_shader_program(vs: GLuint, fs: GLuint) -> Shader {
    let shader;
    let mut uniforms = Vec::new();

    unsafe {
        shader = gl::CreateProgram();
        gl::AttachShader(shader, vs);
        gl::AttachShader(shader, fs);
        gl::LinkProgram(shader);

        // Get link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(shader, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "{}",
                std::str::from_utf8_unchecked(&buf)
            );
        }

        // @Maybe vs and fs can be cached in case we reuse them often, but not likely
        gl::DetachShader(shader, vs);
        gl::DetachShader(shader, fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        // Get uniforms information
        let mut active_uniforms = 0;
        gl::GetProgramiv(shader, gl::ACTIVE_UNIFORMS, &mut active_uniforms);

        for index in 0..active_uniforms {
            let mut length  : GLsizei      = 0;
            let mut size    : GLint        = 0;
            let mut variant : GLenum = 0;

            let mut name = Vec::with_capacity(MAX_NAME_LENGTH);
            name.set_len(MAX_NAME_LENGTH);

            gl::GetActiveUniform(
                shader,
                index as GLuint,
                MAX_NAME_LENGTH as _,
                &mut length,
                &mut size,
                &mut variant,
                name.as_mut_ptr() as *mut GLchar,
            );

            name[length as usize] = 0;
            name.set_len(length as usize);

            let location = gl::GetUniformLocation(shader, name.as_mut_ptr() as *mut GLchar);

            // @TODO (from NoelFB blah) "array names end with [0]"
            let name = String::from_utf8_unchecked(name);

            // @TODO check what NoelFB is doing because he does something different for Sampler2D
            let variant = match variant {
                gl::FLOAT      => UniformVariant::Float,
                gl::FLOAT_VEC2 => UniformVariant::Float2,
                gl::FLOAT_VEC3 => UniformVariant::Float3,
                gl::FLOAT_VEC4 => UniformVariant::Float4,
                gl::FLOAT_MAT4 => UniformVariant::Mat4,
                gl::SAMPLER_2D => UniformVariant::Texture2D,
                _ => {
                    // @TODO logger
                    // @TODO return Result
                    panic!("[renderer][shader] Unsupported uniform type!");
                }
            };

            uniforms.push(UniformInfo {
                name,
                variant,
                location,
            });
        }
    }

    Shader {
        id: shader,
        uniforms,
    }
}

// @TODO return Option/Result (and rename to try_)
fn create_shader_program<P: AsRef<Path>>(vs_path: P, fs_path: P) -> ShaderRef {
    let vs = compile_shader_from_file(vs_path, gl::VERTEX_SHADER);
    let fs = compile_shader_from_file(fs_path, gl::FRAGMENT_SHADER);
    let shader = link_shader_program(vs, fs);
    Rc::new(RefCell::new(shader))
}
