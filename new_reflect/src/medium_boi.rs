use bevy::prelude::Reflect;

#[derive(Default, Reflect)]
pub struct NonGenericMediumBoi {
    field_000: i32,
    field_001: i32,
    field_002: i32,
    field_003: i32,
    field_004: i32,
    field_005: i32,
    field_006: i32,
    field_007: i32,
    field_008: i32,
    field_009: i32,
    field_010: i32,
    field_011: i32,
    field_012: i32,
    field_013: i32,
    field_014: i32,
    field_015: i32,
}

#[derive(Default, Reflect)]
pub struct GenericMediumBoi<T1: Reflect + Default, T2: Reflect + Default, T3: Reflect + Default> {
    field_000: T1,
    field_001: T2,
    field_002: T3,
    field_003: T1,
    field_004: T2,
    field_005: T3,
    field_006: T1,
    field_007: T2,
    field_008: T3,
    field_009: T1,
    field_010: T2,
    field_011: T3,
    field_012: T1,
    field_013: T2,
    field_014: T3,
    field_015: T1,
}