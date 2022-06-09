use bevy::prelude::Reflect;

#[derive(Default, Reflect)]
pub struct NonGenericSmallBoi {
    field_000: i32,
    field_001: i32,
    field_002: i32,
}

#[derive(Default, Reflect)]
pub struct GenericSmallBoi<T1: Reflect + Default, T2: Reflect + Default, T3: Reflect + Default> {
    field_000: T1,
    field_001: T2,
    field_002: T3,
}