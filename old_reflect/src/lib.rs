mod big_boi;
mod medium_boi;
mod small_boi;

use bevy::reflect::{Struct, TypeRegistry, Reflect};
pub use big_boi::*;
pub use medium_boi::*;
pub use small_boi::*;

pub fn prepare<T: Struct>(value: T) -> Box<dyn Struct> {
    let registry = TypeRegistry::default();
    let mut registry = registry.write();
    registry.register::<NonGenericBigBoi>();
    registry.register::<GenericBigBoi<i32, usize, f32>>();

    Box::new(value)
}

pub fn field<'a>(value: &'a dyn Struct, field_name: &str) -> Option<&'a dyn Reflect> {
    value.field(field_name)
}

pub fn field_at<'a>(value: &'a dyn Struct, index: usize) -> Option<&'a dyn Reflect> {
    value.field_at(index)
}