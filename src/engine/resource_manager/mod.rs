use std::collections::HashMap;

use self::{shader::Shader, texture::Texture};

pub mod shader;
pub mod texture;

pub struct ResourceManager {
    shaders: HashMap<&'static str, Shader>,
    textures: HashMap<&'static str, Texture>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
        geometry_shader_path: Option<&str>,
        name: &'static str,
    ) -> Option<Shader> {
        let shader = match Shader::from_source(
            vertex_shader_path,
            fragment_shader_path,
            geometry_shader_path,
        ) {
            Ok(s) => s,
            Err(_) => return None,
        };

        self.shaders.insert(name, shader)
    }

    pub fn get_shader(&self, name: &str) -> Option<Shader> {
        self.shaders.get(name).cloned()
    }

    pub fn load_texture(
        &mut self,
        image_path: &str,
        alpha: bool,
        name: &'static str,
    ) -> Option<Texture> {
        let texture = match Texture::from_image(image_path, alpha) {
            Ok(t) => t,
            Err(_) => return None,
        };

        self.textures.insert(name, texture)
    }

    pub fn get_texture(&self, name: &str) -> Option<Texture> {
        self.textures.get(name).cloned()
    }
}
