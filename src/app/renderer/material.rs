use std::rc::Rc;
use std::cell::RefCell;

use super::*;

#[derive(PartialEq, Debug, ImDraw)]
pub struct Material {
    pub(super) shader: ShaderRef,
    pub(super) uniform_data: Vec<UniformData>,
}

pub type MaterialRef = Rc<RefCell<Material>>;

impl Material {
    pub fn new(shader: ShaderRef) -> MaterialRef {
        // @TODO allocate memory

        let uniform_data = shader.borrow().uniforms.iter()
            .map(|uniform_info| {
                UniformData::from(uniform_info.variant)
            })
            .collect();

        Rc::new(RefCell::new(Material {
            shader: shader.clone(),
            uniform_data,
        }))
    }

    pub fn has_uniform(&self, name: &str) -> bool {
        self.shader.borrow().uniforms.iter()
            .position(|uniform| uniform.name == name)
            .is_some()
    }

    pub fn set_uniform(&mut self, name: &str, value: UniformData) {
        // @TODO return Result
        // Check if exists
        let index = self.shader.borrow().uniforms.iter()
            .position(|uniform| uniform.name == name)
            .unwrap();

        self.uniform_data[index] = value.clone();
    }

    pub fn get_uniform(&self, name: &str) -> Option<(usize, &UniformData)> {
        match self.shader.borrow().uniforms.iter().position(|uniform| uniform.name == name) {
            None => None,
            Some(index) => Some((index, &self.uniform_data[index])),
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<GLint> {
        self.shader.borrow().uniforms.iter()
            .find_map(|uniform| {
                if uniform.name == name {
                    Some(uniform.location)
                } else {
                    None
                }
            })
    }

    /*
    pub fn set_value_at_index(&mut self, index: usize, value: UniformData) {
        // @TODO logger
        assert!(index < self.values.len());
        self.values[index] = value.clone();
    }
    */
}
