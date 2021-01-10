// @TODO Render features
//
// [ ] batch rendering
// [ ] render to framebuffer
// [ ] post processing effects
// [ ] struct Shader
//     [ ] store all uniforms (glGetProgramiv) (do we need to store the attributes also?)
//     [ ] be able to change attribute values during execution
// [ ] Add error checking for gl functions
//

pub mod draw_command;
pub mod texture;
pub mod types;
pub mod color;

use std::ptr;
use std::str;
use std::mem;
use std::ffi::CString;
use std::path::Path;

use crate::linalg::*;

pub use types::*;
pub use draw_command::*;
pub use texture::*;

// TODO move compiling, link
// TODO return Option/Result?
fn compile_shader(src: &str, shader_type: GLenum) -> Shader {
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

fn compile_shader_from_file<P: AsRef<Path>>(path: P, shader_type: GLenum) -> Shader {
    let buffer = std::fs::read_to_string(path)
        //.expect(&format!("File {} not found", path.display()));
        .expect("File not found");

    compile_shader(&buffer, shader_type)
}

fn link_program(vs: Shader, fs: Shader) -> Program {
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

#[derive(Debug)]
pub struct Render {
    current_program: Program,
    current_texture_object: TextureObject,

    view_mat: Mat4,
    proj_mat: Mat4,

    vertex_array_object: VertexArray,

    vertex_buffer_object:  BufferObject,
    color_buffer_object:   BufferObject,
    uv_buffer_object:      BufferObject,
    element_buffer_object: BufferObject,

    /*
    // TODO move to shader info
    texture_uniform:   ShaderLocation,
    model_mat_uniform: ShaderLocation,
    view_mat_uniform:  ShaderLocation,
    proj_mat_uniform:  ShaderLocation,
    */

    // 
    vertex_buffer:  Vec<f32>,
    color_buffer:   Vec<f32>,
    uv_buffer:      Vec<f32>,
    element_buffer: Vec<u32>,

    world_draw_cmds: Vec<DrawCommand>,
}

impl Render {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut bo = [0; 4];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(4, &mut bo[0]);
        }

        let view_mat = mat4::IDENTITY;
        let proj_mat = mat4::ortho(0., 1280., 960., 0.0, 0.01, 1000.);

        // TODO move this
        // Create GLSL shaders
        let vs = compile_shader_from_file(&Path::new("assets/shaders/default.vert"), gl::VERTEX_SHADER);
        let fs = compile_shader_from_file(&Path::new("assets/shaders/default.frag"), gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        Self {
            current_program: program,
            current_texture_object: 0,
            view_mat,
            proj_mat,

            vertex_array_object: vao,
            vertex_buffer_object: bo[0],
            color_buffer_object: bo[1],
            uv_buffer_object: bo[2],
            element_buffer_object: bo[3],

            vertex_buffer: vec![],
            color_buffer: vec![],
            uv_buffer: vec![],
            element_buffer: vec![],

            world_draw_cmds: vec![],
        }
    }

    pub fn prepare_render(&mut self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            //gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            //gl::Enable(gl::DEPTH_TEST);
            //gl::DepthFunc(gl::LEQUAL);
        }
    }

    // @Refactor use a framebuffer to be able to do post processing or custom stuff
    pub fn render_queued_draws(&mut self) {
        if self.world_draw_cmds.len() > 0 {
            self.bind_arrays();
            self.flush_draw_cmds();
        }
    }

    fn bind_arrays(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);

            // positions
            let pos_attr = gl::GetAttribLocation(
                self.current_program,
                CString::new("position").unwrap().as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(pos_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::VertexAttribPointer(
                pos_attr,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null()
            );

            // colors
            let color_attr = gl::GetAttribLocation(
                self.current_program,
                CString::new("color").unwrap().as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(color_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_object);
            gl::VertexAttribPointer(
                color_attr,
                4,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null()
            );

            // uvs
            let uv_attr = gl::GetAttribLocation(
                self.current_program,
                CString::new("uv").unwrap().as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(uv_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_object);
            gl::VertexAttribPointer(
                uv_attr,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null()
            );

            // element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);

            // texture
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    fn flush_draw_cmds(&mut self) {
        self.change_shader_program(self.current_program);

        let mut draw_calls = vec![];
        let mut start = 0usize;

        // XXX it's better if we can move the memory out and clear the vector.
        //     I'm not sure Rust has a way to do this for borrowed variables.
        let draw_cmds = self.world_draw_cmds.to_owned();
        self.world_draw_cmds = vec![];

        //for draw_cmd in self.world_draw_cmds.iter() {
        for draw_cmd in draw_cmds {
            if draw_cmd.program != self.current_program {
                // TODO render_queued_cmds

                //self.change_shader_program(draw_cmd.program);
            }

            let w;
            let h;
            let texture_object;

            match draw_cmd.cmd {
                Command::DrawSprite { size, texture, .. } => {
                    w = size.x;
                    h = size.y;
                    texture_object = texture.obj;
                },
            }

            // HACK do this properly
            let elem = (self.vertex_buffer.len() / 3) as u32;
            self.element_buffer.push(elem + 0);
            self.element_buffer.push(elem + 1);
            self.element_buffer.push(elem + 2);

            self.element_buffer.push(elem + 2);
            self.element_buffer.push(elem + 3);
            self.element_buffer.push(elem + 0);

            // TODO create a 1x1 rect at setup and scale in matrix calculation
            // positions
            /*
            self.vertex_buffer.push(0.); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(1.); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(1.); self.vertex_buffer.push(1.); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(0.); self.vertex_buffer.push(1.); self.vertex_buffer.push(0.);
            */

            self.vertex_buffer.push(0.); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(w); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(w); self.vertex_buffer.push(h); self.vertex_buffer.push(0.);
            self.vertex_buffer.push(0.); self.vertex_buffer.push(h); self.vertex_buffer.push(0.);

            // colors
            self.color_buffer.push(draw_cmd.color.r);
            self.color_buffer.push(draw_cmd.color.g);
            self.color_buffer.push(draw_cmd.color.b);
            self.color_buffer.push(draw_cmd.color.a);

            self.color_buffer.push(draw_cmd.color.r);
            self.color_buffer.push(draw_cmd.color.g);
            self.color_buffer.push(draw_cmd.color.b);
            self.color_buffer.push(draw_cmd.color.a);

            self.color_buffer.push(draw_cmd.color.r);
            self.color_buffer.push(draw_cmd.color.g);
            self.color_buffer.push(draw_cmd.color.b);
            self.color_buffer.push(draw_cmd.color.a);

            self.color_buffer.push(draw_cmd.color.r);
            self.color_buffer.push(draw_cmd.color.g);
            self.color_buffer.push(draw_cmd.color.b);
            self.color_buffer.push(draw_cmd.color.a);

            // uv
            self.uv_buffer.push(0.); self.uv_buffer.push(0.);
            self.uv_buffer.push(1.); self.uv_buffer.push(0.);
            self.uv_buffer.push(1.); self.uv_buffer.push(1.);
            self.uv_buffer.push(0.); self.uv_buffer.push(1.);

            // add draw call
            draw_calls.push(DrawCall {
                start,
                count: 6,
                translation: Vec3 {
                    x: draw_cmd.pos.x,
                    y: draw_cmd.pos.y,
                    z: (draw_cmd.layer as f32) / 10. + 0.1,
                },
                pivot: draw_cmd.pivot,
                rot: draw_cmd.rot,
                texture_object,
            });

            //start += 6;

            // TODO remove this
            self.render_draw_calls(&mut draw_calls);
            start = 0;
        }
    }

    fn render_draw_calls(&mut self, draw_calls: &mut Vec<DrawCall>) {
        self.create_buffer_data();

        // @Refactor do a single draw call here (glDrawElementsIntanced + glVertAttribDivisor)
        for call in draw_calls.iter() {
            if call.texture_object != self.current_texture_object {
                self.change_texture(call.texture_object);
            }

            let model_mat =
                mat4::translation(Vec3 { x: -call.pivot.x, y: -call.pivot.y, z: 0. }) *
                mat4::rotation(call.rot.to_radians(), vec3::FORWARD) *
                mat4::translation(Vec3 {
                    x: call.translation.x,
                    y: call.translation.y,
                    z: call.translation.z,
                });

            unsafe {
                // @TODO send pivot, rotation and translation to shader and do a single draw call
                let model_mat_uniform = gl::GetUniformLocation(
                    self.current_program,
                    CString::new("model_mat").unwrap().as_ptr()
                );

                gl::UniformMatrix4fv(
                    model_mat_uniform,
                    1,
                    gl::FALSE as GLboolean,
                    mem::transmute(&model_mat.m[0])
                );

                gl::DrawElements(
                    gl::TRIANGLES,
                    call.count as i32,
                    gl::UNSIGNED_INT,
                    mem::transmute(call.start * mem::size_of::<GLuint>())
                );
            }
        }

        self.vertex_buffer.clear();
        self.color_buffer.clear();
        self.uv_buffer.clear();
        self.element_buffer.clear();
        draw_calls.clear();
    }

    fn create_buffer_data(&mut self) {
        assert!(!self.vertex_buffer.is_empty());
        assert!(!self.color_buffer.is_empty());
        assert!(!self.uv_buffer.is_empty());
        assert!(!self.element_buffer.is_empty());

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.vertex_buffer[0]),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.color_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.color_buffer[0]),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.uv_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.uv_buffer[0]),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.element_buffer.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                mem::transmute(&self.element_buffer[0]),
                gl::STATIC_DRAW
            );
        }
    }

    fn change_shader_program(&mut self, new_program: Program) {
        //if self.current_program == new_program { return; }

        self.current_program = new_program;

        unsafe {
            gl::UseProgram(self.current_program);

            // TODO verify errors in case names are incorrect
            let texture_uniform = gl::GetUniformLocation(
                self.current_program,
                CString::new("tex").unwrap().as_ptr()
            );

            gl::Uniform1i(texture_uniform, 0);

            let view_mat_uniform = gl::GetUniformLocation(
                self.current_program,
                CString::new("view_mat").unwrap().as_ptr()
            );

            gl::UniformMatrix4fv(
                view_mat_uniform,
                1,
                gl::FALSE as GLboolean,
                &self.view_mat.m[0][0]
            );

            let proj_mat_uniform = gl::GetUniformLocation(
                self.current_program,
                CString::new("proj_mat").unwrap().as_ptr()
            );

            gl::UniformMatrix4fv(
                proj_mat_uniform,
                1,
                gl::FALSE as GLboolean,
                &self.proj_mat.m[0][0]
            );
        }
    }

    fn change_texture(&mut self, new_texture_object: TextureObject) {
        println!("changed texture: {}", new_texture_object);
        self.current_texture_object = new_texture_object;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.current_texture_object);
        }
    }
}

impl Drop for Render {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vertex_array_object);
            gl::DeleteBuffers(1, &mut self.vertex_buffer_object);
            gl::DeleteBuffers(1, &mut self.color_buffer_object);
            gl::DeleteBuffers(1, &mut self.uv_buffer_object);
            gl::DeleteBuffers(1, &mut self.element_buffer_object);
        }
    }
}

// TODO move this
#[derive(Copy, Clone, Debug)]
struct DrawCall {
    start: usize,
    count: usize,
    translation: Vec3,
    pivot: Vec2,
    rot: f32,
    texture_object: TextureObject,
}
