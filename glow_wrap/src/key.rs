use crate::types::GLuint;

pub trait ConvertKey {
    fn from_key(key: GLuint) -> Self;
    fn to_key(&self) -> GLuint;
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::types::GLuint;
    use std::num::NonZeroU32;
    macro_rules! convert_key {
        ($type:path) => {
            impl super::ConvertKey for $type {
                fn from_key(key: GLuint) -> Self {
                    unsafe { Self(NonZeroU32::new_unchecked(key)) }
                }
                fn to_key(&self) -> GLuint {
                    self.0.get()
                }
            }
        };
    }
    convert_key!(glow::NativeBuffer);
    convert_key!(glow::NativeProgram);
    convert_key!(glow::NativeShader);
    convert_key!(glow::NativeTexture);
    convert_key!(glow::NativeVertexArray);

    impl super::ConvertKey for glow::NativeUniformLocation {
        fn from_key(key: crate::types::GLuint) -> Self {
            Self(key)
        }
        fn to_key(&self) -> crate::types::GLuint {
            self.0
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use crate::types::GLuint;
    use slotmap::Key;
    macro_rules! convert_key {
        ($type:path) => {
            impl super::ConvertKey for $type {
                fn from_key(key: GLuint) -> Self {
                    Self::from(slotmap::KeyData::from_ffi((1 as u64) << 32 | key as u64))
                }
                fn to_key(&self) -> GLuint {
                    self.data().as_ffi() as GLuint
                }
            }
        };
    }

    convert_key!(glow::WebBufferKey);
    convert_key!(glow::WebProgramKey);
    convert_key!(glow::WebShaderKey);
    convert_key!(glow::WebTextureKey);
    convert_key!(glow::WebVertexArrayKey);
    convert_key!(crate::WebGlUniformLocationKey);
}
