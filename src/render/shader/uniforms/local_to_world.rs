use crate::{
    asset::Handle,
    render::{
        shader::{AsUniforms, FieldBindType, FieldUniformName},
        texture::Texture,
    },
};

use zerocopy::AsBytes;

const LOCAL_TO_WORLD_FIELD_UNIFORM_NAMES: &[FieldUniformName] = &[FieldUniformName {
    field: "object",
    uniform: "Object",
    texture: "",
    sampler: "",
}];

impl AsUniforms for bevy_transform::prelude::LocalToWorld {
    fn get_field_uniform_names(&self) -> &[FieldUniformName] {
        LOCAL_TO_WORLD_FIELD_UNIFORM_NAMES
    }

    fn get_uniform_bytes(&self, name: &str) -> Option<Vec<u8>> {
        match name {
            "Object" => Some(self.0.as_ref().as_bytes().into()),
            _ => None,
        }
    }

    fn get_shader_defs(&self) -> Option<Vec<String>> {
        None
    }
    fn get_field_bind_type(&self, name: &str) -> Option<FieldBindType> {
        match name {
            "object" => Some(FieldBindType::Uniform),
            _ => None,
        }
    }
    fn get_uniform_texture(&self, _name: &str) -> Option<Handle<Texture>> {
        None
    }

    fn get_uniform_bytes_ref(&self, name: &str) -> Option<&[u8]> {
        match name {
            "Object" => Some(self.0.as_ref().as_bytes()),
            _ => None,
        }
    }
}
