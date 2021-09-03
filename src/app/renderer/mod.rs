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

pub mod color;
pub mod draw_command;
pub mod font;
mod shader;
pub mod sprite;
pub mod texture;

use std::str;
use std::mem;
use std::ffi::CString;
use std::path::Path;
use gl::types::*;
use crate::linalg::*;
use crate::app::{ App, ImDraw };

pub use color::*;
use shader::*;
pub use sprite::*;
pub use texture::*;

use draw_command::{Command, DrawCommand, DrawVariant};

pub type VertexArray    = GLuint;
pub type BufferObject   = GLuint;
pub type ShaderProgram  = GLuint;
pub type Shader         = GLuint;
pub type ShaderLocation = GLuint;

#[derive(Debug, ImDraw)]
pub(in crate::app) struct Renderer {
    default_program: ShaderProgram,
    default_texture: Texture,

    view_mat: Mat4,
    proj_mat: Mat4,

    vertex_array_object: VertexArray,

    vertex_buffer_object:  BufferObject,
    color_buffer_object:   BufferObject,
    uv_buffer_object:      BufferObject,
    element_buffer_object: BufferObject,

    // @Refactor don't use Vec since debug push performance is so bad. We can add a frame allocator
    //           instead and pack the different info in contiguous memory (create a struct and use
    //           offset_of)
    // @Refactor maybe use only one vbo? Not sure the cost of doing this
    vertex_buffer:  Vec<f32>,
    color_buffer:   Vec<f32>,
    uv_buffer:      Vec<f32>,
    element_buffer: Vec<u32>,

    world_cmds: Vec<Command>,

    // ----
    model_mat_buffer_object: BufferObject,
    model_mat_buffer: Vec<f32>,

    // ----
    window_size: (u32, u32),
}

impl Renderer {
    pub(in crate::app) fn new(window_size: (u32, u32)) -> Self {
        let mut vao = 0;
        let mut bo = [0; 5];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(5, &mut bo[0]);
        }

        let view_mat = mat4::IDENTITY;
        let proj_mat = mat4::ortho(
            0., window_size.0 as f32,
            window_size.1 as f32, 0.0,
            0.01, 1000.
        );

        // Create default shader program and texture.
        // These are used when no shader program or texture is passed to a draw command.
        let default_program = create_shader_program("assets/shaders/default.vert", "assets/shaders/default.frag");
        let default_texture = create_texture(&[0xff, 0xff, 0xff, 0xff], 1, 1);

        // Reserve a lot of space -> 2000 quads
        // @TODO use a frame allocator to avoid extra allocations
        let mut vertex_buffer = vec![];
        vertex_buffer.reserve(4 * 3 * 2000);

        let mut color_buffer = vec![];
        color_buffer.reserve(4 * 4 * 2000);

        let mut uv_buffer = vec![];
        uv_buffer.reserve(4 * 2 * 2000);

        let mut element_buffer = vec![];
        element_buffer.reserve(6 * 2000);

        let mut model_mat_buffer = vec![];
        model_mat_buffer.reserve(6 * 2000);

        Self {
            default_program,
            default_texture,

            view_mat,
            proj_mat,

            vertex_array_object: vao,
            vertex_buffer_object: bo[0],
            color_buffer_object: bo[1],
            uv_buffer_object: bo[2],
            element_buffer_object: bo[3],

            vertex_buffer,
            color_buffer,
            uv_buffer,
            element_buffer,

            world_cmds: vec![],

            // ----
            model_mat_buffer_object: bo[4],
            model_mat_buffer,

            window_size,
        }
    }

    // @Refactor create methods in App to remap this
    pub(in crate::app) fn prepare_render(&mut self) {
        unsafe {
            gl::Disable(gl::SCISSOR_TEST);

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
    }

    // @Refactor create methods in App to remap this
    // @Refactor use a framebuffer to be able to do post processing or custom stuff
    fn render_queued(&mut self) {
        if self.world_cmds.len() > 0 {
            //self.bind_arrays();
            self.flush_cmds();
        }
    }

    fn bind_arrays(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);

            // positions
            let pos_cstr = CString::new("position").unwrap();
            let pos_attr = gl::GetAttribLocation(
                self.default_program,
                pos_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(pos_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::VertexAttribPointer(
                pos_attr,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                0 as _
            );

            // colors
            let color_cstr = CString::new("color").unwrap();
            let color_attr = gl::GetAttribLocation(
                self.default_program,
                color_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(color_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_object);
            gl::VertexAttribPointer(
                color_attr,
                4,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                0 as _
            );

            // uvs
            let uv_cstr = CString::new("uv").unwrap();
            let uv_attr = gl::GetAttribLocation(
                self.default_program,
                uv_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(uv_attr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_object);
            gl::VertexAttribPointer(
                uv_attr,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                0 as _
            );

            // model matrix info
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_mat_buffer_object);

            let pivot_cstr = CString::new("pivot").unwrap();
            let pivot_attr = gl::GetAttribLocation(
                self.default_program,
                pivot_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(pivot_attr);
            gl::VertexAttribPointer(
                pivot_attr,
                2,
                gl::FLOAT,
                gl::FALSE,
                (6 * mem::size_of::<GLfloat>()) as _,
                0 as _
            );

            let rotation_cstr = CString::new("rotation").unwrap();
            let rotation_attr = gl::GetAttribLocation(
                self.default_program,
                rotation_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(rotation_attr);
            gl::VertexAttribPointer(
                rotation_attr,
                1,
                gl::FLOAT,
                gl::FALSE,
                (6 * mem::size_of::<GLfloat>()) as _,
                (2 * mem::size_of::<GLfloat>()) as _
            );

            let translation_cstr = CString::new("translation").unwrap();
            let translation_attr = gl::GetAttribLocation(
                self.default_program,
                translation_cstr.as_ptr()
            ) as ShaderLocation;

            gl::EnableVertexAttribArray(translation_attr);
            gl::VertexAttribPointer(
                translation_attr,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * mem::size_of::<GLfloat>()) as _,
                (3 * mem::size_of::<GLfloat>()) as _
            );

            // element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);

            // texture
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    fn handle_draw_command(&mut self, cmd: DrawCommand, draw_call: &mut DrawCall) {
        let w;
        let h;
        let pivot;

        //let mut us = vec![0., 1., 1., 0.];
        //let mut vs = vec![0., 0., 1., 1.];
        let mut us;
        let mut vs;

        match cmd.variant {
            DrawVariant::Solid { size } => {
                pivot = Vec2::new();
                w = size.x * cmd.scale.x;
                h = size.y * cmd.scale.y;

                us = vec![0.0, 1.0, 1.0, 0.0];
                vs = vec![0.0, 0.0, 1.0, 1.0];
            },

            DrawVariant::Sprite { texture_flip, uvs, pivot: piv, size } => {
                pivot = Vec2 { x: piv.x * cmd.scale.x, y: piv.y * cmd.scale.y };
                w = size.x * cmd.scale.x;
                h = size.y * cmd.scale.y;

                let u_scale = if cmd.texture.w != 0 { cmd.texture.w as f32 } else { 1. };
                let v_scale = if cmd.texture.h != 0 { cmd.texture.h as f32 } else { 1. };

                us = vec![
                    uvs.0.x as f32 / u_scale, uvs.1.x as f32 / u_scale,
                    uvs.1.x as f32 / u_scale, uvs.0.x as f32 / u_scale,
                ];

                vs = vec![
                    uvs.0.y as f32 / v_scale, uvs.0.y as f32 / v_scale,
                    uvs.1.y as f32 / v_scale, uvs.1.y as f32 / v_scale,
                ];

                if texture_flip.contains(TextureFlip::X) { us.swap(0, 1); us.swap(2, 3); }
                if texture_flip.contains(TextureFlip::Y) { vs.swap(0, 2); vs.swap(1, 3); }
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
        self.color_buffer.push(cmd.color.r);
        self.color_buffer.push(cmd.color.g);
        self.color_buffer.push(cmd.color.b);
        self.color_buffer.push(cmd.color.a);

        self.color_buffer.push(cmd.color.r);
        self.color_buffer.push(cmd.color.g);
        self.color_buffer.push(cmd.color.b);
        self.color_buffer.push(cmd.color.a);

        self.color_buffer.push(cmd.color.r);
        self.color_buffer.push(cmd.color.g);
        self.color_buffer.push(cmd.color.b);
        self.color_buffer.push(cmd.color.a);

        self.color_buffer.push(cmd.color.r);
        self.color_buffer.push(cmd.color.g);
        self.color_buffer.push(cmd.color.b);
        self.color_buffer.push(cmd.color.a);

        // uv
        self.uv_buffer.push(us[0]); self.uv_buffer.push(vs[0]);
        self.uv_buffer.push(us[1]); self.uv_buffer.push(vs[1]);
        self.uv_buffer.push(us[2]); self.uv_buffer.push(vs[2]);
        self.uv_buffer.push(us[3]); self.uv_buffer.push(vs[3]);

        // model matrix info
        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(cmd.rot);
        self.model_mat_buffer.push(cmd.pos.x); self.model_mat_buffer.push(cmd.pos.y); self.model_mat_buffer.push((cmd.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(cmd.rot);
        self.model_mat_buffer.push(cmd.pos.x); self.model_mat_buffer.push(cmd.pos.y); self.model_mat_buffer.push((cmd.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(cmd.rot);
        self.model_mat_buffer.push(cmd.pos.x); self.model_mat_buffer.push(cmd.pos.y); self.model_mat_buffer.push((cmd.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(cmd.rot);
        self.model_mat_buffer.push(cmd.pos.x); self.model_mat_buffer.push(cmd.pos.y); self.model_mat_buffer.push((cmd.layer as f32) / 10. + 0.1);

        draw_call.count += 6;
    }

    fn flush_cmds(&mut self) {
        if self.world_cmds.is_empty() {
            return;
        }

        let mut draw_calls = vec![];

        let mut draw_call = DrawCall {
            program: self.default_program,
            texture_object: self.default_texture.obj,
            clip: None,
            start: 0,
            count: 0,
        };

        let mut clip_stack: Vec<(Vec2i, Vec2i)> = Vec::new();

        let cmds = std::mem::take(&mut self.world_cmds);
        for cmd in cmds {
            match cmd {
                Command::Draw(cmd) => {
                    // @TODO remove the zero check after we have access to programs outside render
                    if cmd.program != draw_call.program || cmd.texture.obj != draw_call.texture_object {
                        add_draw_call(&mut draw_call, &mut draw_calls);

                        draw_call.program = cmd.program;
                        draw_call.texture_object = cmd.texture.obj;
                    }

                    self.handle_draw_command(cmd, &mut draw_call);
                }

                Command::PushClip { mut min, mut max, intersect } => {
                    add_draw_call(&mut draw_call, &mut draw_calls);

                    if intersect && !clip_stack.is_empty() {
                        let top = clip_stack.last().unwrap();
                        min.x = std::cmp::max(min.x, top.0.x);
                        min.y = std::cmp::max(min.y, top.0.y);

                        max.x = std::cmp::min(max.x, top.1.x);
                        max.y = std::cmp::min(max.y, top.1.y);
                    }

                    clip_stack.push((min, max));
                    draw_call.clip = Some((min, max));
                }

                Command::PopClip => {
                    add_draw_call(&mut draw_call, &mut draw_calls);

                    clip_stack.pop().expect("clip pop with no push");
                    draw_call.clip = clip_stack.last().cloned();
                }
            }
        }

        add_draw_call(&mut draw_call, &mut draw_calls);
        self.render_draw_calls(draw_calls);
    }

    fn render_draw_calls(&mut self, draw_calls: Vec<DrawCall>) {
        if draw_calls.is_empty() { return; }

        self.change_shader_program(draw_calls[0].program);
        self.change_texture(draw_calls[0].texture_object);

        self.bind_arrays();
        self.create_buffer_data();

        // @TODO improve this, somehow
        let mut current_program = draw_calls[0].program;
        let mut current_texture_object = draw_calls[0].texture_object;

        // @Refactor do a single draw call here (glDrawElementsIntanced + glVertexAttribDivisor)
        for call in draw_calls.iter() {
            // @TODO remove the zero check after we have access to programs outside render
            if call.program != current_program {
                self.change_shader_program(call.program);
                current_program = call.program;
            }

            if call.texture_object != current_texture_object {
                self.change_texture(call.texture_object);
                current_texture_object = call.texture_object;
            }

            if let Some((min, max)) = call.clip {
                let x = min.x;
                let y = self.window_size.1 as i32 - max.y;
                let w = max.x - min.x;
                let h = max.y - min.y;

                unsafe {
                    gl::Enable(gl::SCISSOR_TEST);
                    gl::Scissor(x, y, w, h);
                }
            } else {
                unsafe {
                    gl::Disable(gl::SCISSOR_TEST);
                }
            }

            unsafe {
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
        self.model_mat_buffer.clear();
    }

    fn create_buffer_data(&mut self) {
        if self.vertex_buffer.is_empty() {
            return;
        }

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

            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_mat_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.model_mat_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.model_mat_buffer[0]),
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

    fn change_shader_program(&mut self, new_program: ShaderProgram) {
        unsafe {
            gl::UseProgram(new_program);

            // TODO verify errors in case names are incorrect
            let texture_uniform_cstr = CString::new("tex").unwrap();
            let texture_uniform = gl::GetUniformLocation(
                new_program,
                texture_uniform_cstr.as_ptr()
            );

            gl::Uniform1i(texture_uniform, 0);

            let view_mat_cstr = CString::new("view_mat").unwrap();
            let view_mat_uniform = gl::GetUniformLocation(
                new_program,
                view_mat_cstr.as_ptr()
            );

            gl::UniformMatrix4fv(
                view_mat_uniform,
                1,
                gl::FALSE as GLboolean,
                &self.view_mat.m[0][0]
            );

            let proj_mat_cstr = CString::new("proj_mat").unwrap();
            let proj_mat_uniform = gl::GetUniformLocation(
                new_program,
                proj_mat_cstr.as_ptr()
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
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, new_texture_object);
        }
    }

    pub(in crate::app) fn window_resize_callback(&mut self, window_size: (u32, u32)) {
        self.proj_mat = mat4::ortho(
            0., window_size.0 as f32,
            window_size.1 as f32, 0.0,
            0.01, 1000.
        );
        self.window_size = window_size;

        unsafe {
            gl::Viewport(0, 0, window_size.0 as _, window_size.1 as _);
        }
    }
}

impl Drop for Renderer {
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

impl App<'_> {
    pub fn render_queued(&mut self) {
        self.renderer.render_queued();
    }
}

// TODO move this
#[derive(Copy, Clone, Debug)]
struct DrawCall {
    program: ShaderProgram,
    texture_object: TextureObject,
    clip: Option<(Vec2i, Vec2i)>,
    start: usize,
    count: usize,
}


fn add_draw_call(draw_call: &mut DrawCall, draw_calls: &mut Vec<DrawCall>) {
    if draw_call.count != 0 {
        draw_calls.push(*draw_call);
        draw_call.start += draw_call.count;
        draw_call.count = 0;
    }
}
