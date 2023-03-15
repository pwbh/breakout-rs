use std::{ffi::CString, fs::read_to_string};

use gl::types::*;

#[derive(Clone, Copy)]
pub struct Shader {
    program_id: GLuint,
}

impl Shader {
    pub fn from_source(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
        geometry_shader_path: Option<&str>,
    ) -> Result<Shader, String> {
        let vertex_shader_id = compile_shader(vertex_shader_path, gl::VERTEX_SHADER)?;
        let fragment_shader_id = compile_shader(fragment_shader_path, gl::FRAGMENT_SHADER)?;

        let geometry_shader_id = match geometry_shader_path {
            Some(path) => Some(compile_shader(path, gl::GEOMETRY_SHADER)?),
            None => None,
        };

        let program_id = link_shaders(vertex_shader_id, fragment_shader_id, geometry_shader_id)?;

        Ok(Self { program_id })
    }

    pub fn to_use(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let name = CString::new(name).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr());
            gl::Uniform1i(location, value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let name = CString::new(name).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let name = CString::new(name).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_mat4(&self, name: &str, value: *const f32) {
        let name = CString::new(name).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value);
        }
    }

    pub fn set_vec3(&self, name: &str, value: glam::Vec3) {
        let name = CString::new(name).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }
}

fn compile_shader(path: &str, kind: GLuint) -> Result<GLuint, String> {
    let source = match read_to_string(path) {
        Ok(s) => match CString::new(s) {
            Ok(c_str) => c_str,
            Err(_) => {
                return Err(String::from(
                    "Couldn't convert source to 0 terminated String (CString)",
                ))
            }
        },
        Err(_e) => return Err(String::from("Couldn't open the shader source")),
    };

    let shader_id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);
    }

    let mut success: GLint = 1;

    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;

        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(shader_id)
}

fn link_shaders(
    vertex_shader_id: GLuint,
    fragment_shader_id: GLuint,
    geometry_shader_id: Option<GLuint>,
) -> Result<GLuint, String> {
    let program_id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program_id, vertex_shader_id);
        gl::AttachShader(program_id, fragment_shader_id);

        if let Some(geomatry_shader_id) = geometry_shader_id {
            gl::AttachShader(program_id, geomatry_shader_id)
        }
    }

    unsafe { gl::LinkProgram(program_id) }

    let mut success: GLint = 1;

    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;

        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    unsafe {
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);

        if let Some(geomatry_shader_id) = geometry_shader_id {
            gl::DeleteShader(geomatry_shader_id)
        }
    }

    Ok(program_id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![b' '; len + 1]) }
}
