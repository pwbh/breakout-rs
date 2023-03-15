use gl::types::*;

use super::resource_manager::{shader::Shader, texture::Texture};

pub struct SpriteRenderer {
    shader: Shader,
    quad_vao: GLuint,
}

impl SpriteRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let projection =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        let mut quad_vao = 0;
        let mut vbo = 0;

        let vertices: [f32; 24] = [
            // pos    // tex
            0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut quad_vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * vertices.len()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(quad_vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as GLsizei,
                std::ptr::null(),
            )
        }

        let shader = Shader::from_source(
            "./src/engine/shaders/sprite.vert",
            "./src/engine/shaders/sprite.frag",
            None,
        )
        .unwrap();

        shader.to_use();
        shader.set_int("image", 0);
        shader.set_mat4("projection", &projection.to_cols_array()[0]);

        Self { quad_vao, shader }
    }

    pub fn draw_sprite(
        &self,
        texture: Texture,
        position: glam::Vec2,
        size: glam::Vec2,
        rotate: f32,
        color: glam::Vec3,
    ) {
        self.shader.to_use();
        let model = glam::Mat4::IDENTITY
            * glam::Mat4::from_translation(glam::vec3(position.x, position.y, 0.0))
            * glam::Mat4::from_translation(glam::vec3(0.5 * size.x, 0.5 * size.y, 0.0))
            * glam::Mat4::from_rotation_z(rotate.to_radians())
            * glam::Mat4::from_translation(glam::vec3(-0.5 * size.x, -0.5 * size.y, 0.0))
            * glam::Mat4::from_scale(glam::vec3(size.x, size.y, 1.0));

        self.shader.set_mat4("model", &model.to_cols_array()[0]);
        self.shader.set_vec3("spriteColor", color);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }

        texture.bind();

        unsafe {
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}
