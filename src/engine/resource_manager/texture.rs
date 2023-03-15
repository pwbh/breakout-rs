use gl::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn from_image(image_path: &str, alpha: bool) -> Result<Self, String> {
        let format = if alpha { gl::RGBA } else { gl::RGB };

        let image = match image::open(image_path) {
            Ok(i) => i,
            Err(e) => return Err(e.to_string()),
        };

        let mut texture: GLuint = 0;

        unsafe {
            // Create texture
            gl::GenTextures(1, &mut texture);
            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                image.width() as GLsizei,
                image.height() as GLsizei,
                0,
                format,
                gl::UNSIGNED_BYTE,
                image.as_bytes().as_ptr() as *const _,
            );
            // Set texture wrap and filter modes
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            // Unbind the current texture, will be rebind whenever we want to draw to the screen
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Self { id: texture })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
