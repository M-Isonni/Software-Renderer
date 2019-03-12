use gl;
use std::ffi::{CString, CStr};


pub fn shader_from_source(source: &CStr,kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    
    unsafe {
    gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    gl::CompileShader(id);
    };

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    };

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len as usize));
        // convert buffer to CString
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
        unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)

}