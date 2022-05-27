//#[macro_use] pub mod batch;
pub mod batch;
pub mod color;
mod draw_call;
mod draw_command;
pub mod framebuffer;
pub mod material;
pub mod subtexture;
pub mod shader;
pub mod sprite;
pub mod text;
pub mod texture;
#[macro_use] mod vertex_format;

use std::ffi::CString;
use gl::types::*;
use crate::linalg::*;
use crate::app::{App, ImDraw};

pub use batch::*;
pub use color::*;
use draw_call::*;
use draw_command::*;
pub use framebuffer::*;
pub use material::*;
pub use subtexture::*;
pub use shader::*;
pub use sprite::*;
pub use texture::*;
use vertex_format::*;

pub(in crate::app) type VertexArray  = GLuint;
pub(in crate::app) type BufferObject = GLuint;
pub(in crate::app) type ShaderObject = GLuint;
pub(in crate::app) type Location     = GLuint;

#[derive(Debug, ImDraw)]
pub(in crate::app) struct Renderer {
    default_material: MaterialRef,
    default_shader:   ShaderRef,
    default_texture:  TextureRef,

    vertex_format: VertexFormat,

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

    vertex_count: u32,

    batch: Batch,

    // ----
    model_mat_buffer_object: BufferObject,
    model_mat_buffer: Vec<f32>,

    // ----
    window_size: (u32, u32),
}

impl App<'_> {
    pub fn batch(&mut self) -> &mut Batch { &mut self.renderer.batch }
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

        // Create default shader program, texture and material
        // These are used when no shader program or texture is passed to a draw command.
        let default_shader = Shader::new("assets/shaders/default.vert", "assets/shaders/default.frag");

        let default_texture = Texture::new(1, 1, Some(&[0xff, 0xff, 0xff, 0xff]));
        default_texture.borrow_mut().set_white_pixel((0, 0));

        let default_material = Material::new(default_shader.clone());
        default_material.borrow_mut()
            .set_uniform("u_texture", UniformData::Texture2D(default_texture.clone()));

        // I would love to be able to not fix this, but it seems very unlikely that I can and should
        let vertex_format = VertexFormat::new(vec![
            VertexAttribute { location: 0, variant: AttributeVariant::Float3, normalized: false },
            VertexAttribute { location: 1, variant: AttributeVariant::Float4, normalized: false },
            VertexAttribute { location: 2, variant: AttributeVariant::Float2, normalized: false },
        ]);

        // Reserve a lot of space -> 2000 quads
        // @TODO use a frame allocator to avoid extra allocations
        let vertex_buffer    = Vec::with_capacity(4 * 3 * 2000);
        let color_buffer     = Vec::with_capacity(4 * 4 * 2000);
        let uv_buffer        = Vec::with_capacity(4 * 2 * 2000);
        let element_buffer   = Vec::with_capacity(6 * 2000);
        let model_mat_buffer = Vec::with_capacity(6 * 2000);

        Self {
            default_material,
            default_shader,
            default_texture,

            vertex_format,

            view_mat,
            proj_mat,

            vertex_array_object:   vao,
            vertex_buffer_object:  bo[0],
            color_buffer_object:   bo[1],
            uv_buffer_object:      bo[2],
            element_buffer_object: bo[3],

            vertex_buffer,
            color_buffer,
            uv_buffer,
            element_buffer,

            vertex_count: 0,

            batch: Batch::new(),

            // ----
            model_mat_buffer_object: bo[4],
            model_mat_buffer,

            window_size,
        }
    }

    pub(in crate::app) fn prepare_render() {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::Disable(gl::SCISSOR_TEST);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn render_batch(&mut self, batch: Batch, framebuffer: Option<FramebufferRef>) {
        self.flush_cmds(batch, framebuffer);
    }

    fn render_queued(&mut self) {
        let batch = std::mem::take(&mut self.batch);
        self.render_batch(batch, None);
    }

    fn bind_arrays(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);

            // Setup vertex format

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            self.vertex_format.attributes().iter().fold(0, |offset, vertex_attrib| {
                let location = vertex_attrib.location as _;
                gl::EnableVertexAttribArray(location);

                let normalized = if vertex_attrib.normalized {
                    gl::TRUE
                } else {
                    gl::FALSE
                };

                gl::VertexAttribPointer(
                    location,
                    vertex_attrib.variant.components_count() as _,
                    gl::FLOAT,
                    normalized,
                    self.vertex_format.total_size() as _,
                    offset as _,
                );

                offset + vertex_attrib.variant.size()
            });

            // @TODO remove model matrix from vertex attributes

            // model matrix info
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_mat_buffer_object);

            let pivot_cstr = CString::new("pivot").unwrap();
            let pivot_attr = gl::GetAttribLocation(
                self.default_shader.borrow().id,
                pivot_cstr.as_ptr()
            ) as Location;

            gl::EnableVertexAttribArray(pivot_attr);
            gl::VertexAttribPointer(
                pivot_attr,
                2,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<GLfloat>()) as _,
                0 as _
            );

            let rotation_cstr = CString::new("rotation").unwrap();
            let rotation_attr = gl::GetAttribLocation(
                self.default_shader.borrow().id,
                rotation_cstr.as_ptr()
            ) as Location;

            gl::EnableVertexAttribArray(rotation_attr);
            gl::VertexAttribPointer(
                rotation_attr,
                1,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<GLfloat>()) as _,
                (2 * std::mem::size_of::<GLfloat>()) as _
            );

            let translation_cstr = CString::new("translation").unwrap();
            let translation_attr = gl::GetAttribLocation(
                self.default_shader.borrow().id,
                translation_cstr.as_ptr()
            ) as Location;

            gl::EnableVertexAttribArray(translation_attr);
            gl::VertexAttribPointer(
                translation_attr,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<GLfloat>()) as _,
                (3 * std::mem::size_of::<GLfloat>()) as _
            );

            // element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);

            // texture
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    fn handle_draw_command(
        &mut self,
        draw_command_data: DrawCommandData,
        mut draw_call: &mut DrawCall,
        mut draw_calls: &mut Vec<DrawCall>,
    ) {
        // Get material or use default
        let material = draw_command_data.material.clone()
            .unwrap_or(self.default_material.clone());

        let texture = draw_command_data.texture.or_else(|| {
            if draw_call.texture.borrow().white_pixel.is_some() {
                Some(draw_call.texture.clone())
            } else {
                Some(self.default_texture.clone())
            }
        }).unwrap();

        // @Refactor these borrows seems very bad. Maybe DrawCall should be the most GPU-oriented as
        //           possible, only storing the GPU object ids for textures and shaders
        if material != draw_call.material || texture != draw_call.texture {
            add_draw_call(&mut draw_call, &mut draw_calls);
            draw_call.material = material;
            draw_call.texture = texture;
        }


        let texture = draw_call.texture.borrow();
        let u_scale_inv = if texture.w != 0 { 1. / texture.w as f32 } else { 1. };
        let v_scale_inv = if texture.h != 0 { 1. / texture.h as f32 } else { 1. };

        let size = draw_command_data.size;

        let w;
        let h;
        let pivot;

        let mut us;
        let mut vs;

        match draw_command_data.variant {
            DrawVariant::Solid => {
                pivot = Vec2::new();
                w = size.x * draw_command_data.transform.scale.x;
                h = size.y * draw_command_data.transform.scale.y;

                let white_pixel_pos = texture.white_pixel.unwrap();

                //us = vec![0.0, 1.0, 1.0, 0.0];
                //vs = vec![0.0, 0.0, 1.0, 1.0];

                us = vec![
                    white_pixel_pos.0 as f32 * u_scale_inv,
                    (white_pixel_pos.0 + 1) as f32 * u_scale_inv,
                    (white_pixel_pos.0 + 1) as f32 * u_scale_inv,
                    white_pixel_pos.0 as f32 * u_scale_inv,
                ];

                vs = vec![
                    white_pixel_pos.1 as f32 * u_scale_inv,
                    white_pixel_pos.1 as f32 * u_scale_inv,
                    (white_pixel_pos.1 + 1) as f32 * u_scale_inv,
                    (white_pixel_pos.1 + 1) as f32 * u_scale_inv,
                ];
            },

            DrawVariant::Sprite { texture_flip, uvs, pivot: piv } => {
                pivot = Vec2 { x: piv.x * draw_command_data.transform.scale.x, y: piv.y * draw_command_data.transform.scale.y };
                w = size.x * draw_command_data.transform.scale.x;
                h = size.y * draw_command_data.transform.scale.y;

                us = vec![
                    uvs.0.x as f32 * u_scale_inv, uvs.1.x as f32 * u_scale_inv,
                    uvs.1.x as f32 * u_scale_inv, uvs.0.x as f32 * u_scale_inv,
                ];

                vs = vec![
                    uvs.0.y as f32 * v_scale_inv, uvs.0.y as f32 * v_scale_inv,
                    uvs.1.y as f32 * v_scale_inv, uvs.1.y as f32 * v_scale_inv,
                ];

                if texture_flip.contains(TextureFlip::X) { us.swap(0, 1); us.swap(2, 3); }
                if texture_flip.contains(TextureFlip::Y) { vs.swap(0, 2); vs.swap(1, 3); }
            },
        }

        // Vertex: pos (float3), color (float4), uv (float2)
        self.vertex_buffer.push(0.); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
        self.vertex_buffer.push(draw_command_data.color.r);
        self.vertex_buffer.push(draw_command_data.color.g);
        self.vertex_buffer.push(draw_command_data.color.b);
        self.vertex_buffer.push(draw_command_data.color.a);
        self.vertex_buffer.push(us[0]); self.vertex_buffer.push(vs[0]);

        self.vertex_buffer.push(w); self.vertex_buffer.push(0.); self.vertex_buffer.push(0.);
        self.vertex_buffer.push(draw_command_data.color.r);
        self.vertex_buffer.push(draw_command_data.color.g);
        self.vertex_buffer.push(draw_command_data.color.b);
        self.vertex_buffer.push(draw_command_data.color.a);
        self.vertex_buffer.push(us[1]); self.vertex_buffer.push(vs[1]);

        self.vertex_buffer.push(w); self.vertex_buffer.push(h); self.vertex_buffer.push(0.);
        self.vertex_buffer.push(draw_command_data.color.r);
        self.vertex_buffer.push(draw_command_data.color.g);
        self.vertex_buffer.push(draw_command_data.color.b);
        self.vertex_buffer.push(draw_command_data.color.a);
        self.vertex_buffer.push(us[2]); self.vertex_buffer.push(vs[2]);

        self.vertex_buffer.push(0.); self.vertex_buffer.push(h); self.vertex_buffer.push(0.);
        self.vertex_buffer.push(draw_command_data.color.r);
        self.vertex_buffer.push(draw_command_data.color.g);
        self.vertex_buffer.push(draw_command_data.color.b);
        self.vertex_buffer.push(draw_command_data.color.a);
        self.vertex_buffer.push(us[3]); self.vertex_buffer.push(vs[3]);

        self.element_buffer.push(self.vertex_count + 0);
        self.element_buffer.push(self.vertex_count + 1);
        self.element_buffer.push(self.vertex_count + 2);

        self.element_buffer.push(self.vertex_count + 2);
        self.element_buffer.push(self.vertex_count + 3);
        self.element_buffer.push(self.vertex_count + 0);
        self.vertex_count += 4;

        // model matrix info
        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(draw_command_data.transform.rot);
        self.model_mat_buffer.push(draw_command_data.transform.pos.x);
        self.model_mat_buffer.push(draw_command_data.transform.pos.y);
        self.model_mat_buffer.push((draw_command_data.transform.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(draw_command_data.transform.rot);
        self.model_mat_buffer.push(draw_command_data.transform.pos.x);
        self.model_mat_buffer.push(draw_command_data.transform.pos.y);
        self.model_mat_buffer.push((draw_command_data.transform.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(draw_command_data.transform.rot);
        self.model_mat_buffer.push(draw_command_data.transform.pos.x);
        self.model_mat_buffer.push(draw_command_data.transform.pos.y);
        self.model_mat_buffer.push((draw_command_data.transform.layer as f32) / 10. + 0.1);

        self.model_mat_buffer.push(pivot.x); self.model_mat_buffer.push(pivot.y);
        self.model_mat_buffer.push(draw_command_data.transform.rot);
        self.model_mat_buffer.push(draw_command_data.transform.pos.x);
        self.model_mat_buffer.push(draw_command_data.transform.pos.y);
        self.model_mat_buffer.push((draw_command_data.transform.layer as f32) / 10. + 0.1);

        draw_call.count += 6;
    }

    fn flush_cmds(&mut self, batch: Batch, framebuffer: Option<FramebufferRef>) {
        if batch.cmds.is_empty() {
            return;
        }

        let mut draw_calls = vec![];

        let mut draw_call = DrawCall {
            material: self.default_material.clone(),
            texture: self.default_texture.clone(),
            clip: None,
            start: 0,
            count: 0,
        };

        let mut clip_stack: Vec<(Vec2i, Vec2i)> = Vec::new();

        for cmd in batch.cmds {
            match cmd {
                DrawCommand::Draw(draw_command_data) => {
                    self.handle_draw_command(draw_command_data, &mut draw_call, &mut draw_calls);
                }

                DrawCommand::PushClip { mut min, mut max, intersect } => {
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

                DrawCommand::PopClip => {
                    add_draw_call(&mut draw_call, &mut draw_calls);

                    clip_stack.pop().expect("clip pop with no push");
                    draw_call.clip = clip_stack.last().cloned();
                }
            }
        }

        add_draw_call(&mut draw_call, &mut draw_calls);
        self.render_draw_calls(draw_calls, framebuffer);
    }

    fn render_draw_calls(&mut self, draw_calls: Vec<DrawCall>, framebuffer: Option<FramebufferRef>) {
        if draw_calls.is_empty() { return; }

        // @Hack
        // @TODO store the correct value already on batch?
        let framebuffer_height;

        // @TODO improve this whole framebuffer logic
        if let Some(framebuffer) = framebuffer {
            framebuffer.borrow_mut().bind(self);
            let height = framebuffer.borrow().height;
            framebuffer_height = height;
        } else {
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

                // @Hack we enable depth test for default. We should move the whole default
                //       framebuffer rendering to a multi-pass rendering using Framebuffer
                gl::Enable(gl::DEPTH_TEST);
            }
            self.change_viewport(self.window_size);
            framebuffer_height = self.window_size.1;
        }

        self.change_shader(draw_calls[0].material.borrow().shader.borrow().id);
        self.change_texture(draw_calls[0].texture.borrow().obj);

        self.bind_arrays();
        self.create_buffer_data();

        // @TODO improve this, somehow
        let mut current_material = draw_calls[0].material.clone();
        let mut current_texture  = draw_calls[0].texture.clone();

        // @Refactor do a single draw call here (glDrawElementsIntanced + glVertexAttribDivisor)
        for call in draw_calls.iter() {
            if call.material != current_material {
                // @TODO bind material uniforms
                // @TODO change_shader -> change_material
                self.change_shader(call.material.borrow().shader.borrow().id);
                current_material = call.material.clone();
            }

            if call.texture != current_texture {
                self.change_texture(call.texture.borrow().obj);
                current_texture = call.texture.clone();
            }

            if let Some((min, max)) = call.clip {
                let x = min.x;
                let y = framebuffer_height as i32 - max.y;
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
                    std::mem::transmute(call.start * std::mem::size_of::<GLuint>())
                );
            }
        }

        self.vertex_buffer.clear();
        self.color_buffer.clear();
        self.uv_buffer.clear();
        self.element_buffer.clear();
        self.model_mat_buffer.clear();
        self.vertex_count = 0;
    }

    fn create_buffer_data(&mut self) {
        if self.vertex_buffer.is_empty() {
            return;
        }

        assert!(!self.vertex_buffer.is_empty());
        //assert!(!self.color_buffer.is_empty());
        //assert!(!self.uv_buffer.is_empty());
        assert!(!self.element_buffer.is_empty());

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertex_buffer.len() * std::mem::size_of::<GLfloat>()) as _,
                std::mem::transmute(&self.vertex_buffer[0]),
                gl::DYNAMIC_DRAW
            );

            /*
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.color_buffer.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                std::mem::transmute(&self.color_buffer[0]),
                gl::DYNAMIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.uv_buffer.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                std::mem::transmute(&self.uv_buffer[0]),
                gl::DYNAMIC_DRAW
            );
            */

            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_mat_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.model_mat_buffer.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                std::mem::transmute(&self.model_mat_buffer[0]),
                gl::DYNAMIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.element_buffer.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                std::mem::transmute(&self.element_buffer[0]),
                gl::DYNAMIC_DRAW
            );
        }
    }

    fn change_shader(&mut self, new_shader_obj: ShaderObject) {
        unsafe {
            gl::UseProgram(new_shader_obj);

            // TODO verify errors in case names are incorrect
            let texture_uniform_cstr = CString::new("u_texture").unwrap();
            let texture_uniform = gl::GetUniformLocation(
                new_shader_obj,
                texture_uniform_cstr.as_ptr()
            );

            gl::Uniform1i(texture_uniform, 0);

            let view_mat_cstr = CString::new("u_view_mat").unwrap();
            let view_mat_uniform = gl::GetUniformLocation(
                new_shader_obj,
                view_mat_cstr.as_ptr()
            );

            gl::UniformMatrix4fv(
                view_mat_uniform,
                1,
                gl::FALSE as GLboolean,
                &self.view_mat.m[0][0]
            );

            let proj_mat_cstr = CString::new("u_proj_mat").unwrap();
            let proj_mat_uniform = gl::GetUniformLocation(
                new_shader_obj,
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

    fn change_viewport(&mut self, size: (u32, u32)) {
        self.proj_mat = mat4::ortho(
            0., size.0 as f32,
            size.1 as f32, 0.0,
            0.01, 1000.
        );

        unsafe {
            gl::Viewport(0, 0, size.0 as _, size.1 as _);
        }
    }

    pub(in crate::app) fn window_resize_callback(&mut self, window_size: (u32, u32)) {
        self.window_size = window_size;
        self.change_viewport(window_size);
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
    pub fn render_batch(&mut self, batch: Batch, framebuffer: Option<FramebufferRef>) {
        self.renderer.render_batch(batch, framebuffer);
    }

    pub(in crate::app) fn render_queued(&mut self) {
        self.renderer.render_queued();
    }
}

fn add_draw_call(draw_call: &mut DrawCall, draw_calls: &mut Vec<DrawCall>) {
    if draw_call.count != 0 {
        draw_calls.push(draw_call.clone());
        draw_call.start += draw_call.count;
        draw_call.count = 0;
    }
}
