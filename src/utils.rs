use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct RelEntity(pub Entity);

#[derive(Component)]
pub struct Music;

#[derive(Component)]
pub struct Sound;

pub struct GameColor;

/// Percent value, 1 is 100%
pub type PercentBw0And1 = f32;

impl GameColor {
    pub const WHITE: Color = Color::srgb(1., 254. / 255., 236. / 255.);
    pub const GRAY: Color = Color::srgb(65. / 255., 69. / 255., 72. / 255.);
    pub const GOLD: Color = Color::srgb(188. / 255., 148. / 255., 87. / 255.);
    pub const BACKGROUND: Color = Color::srgba(23. / 255., 24. / 255., 26. / 255., 120. / 255.);
}

pub mod reflect {
    use bevy::reflect::{Reflect, Struct};

    pub fn prop_name(obj: &impl Struct, i: usize) -> String {
        obj.name_at(i)
            .unwrap_or_else(|| panic!("😭 No name at index {i}"))
            .to_string()
    }

    pub fn get_field_mut(obj: &mut impl Struct, i: usize) -> &mut dyn Reflect {
        obj.field_at_mut(i)
            .unwrap_or_else(|| panic!("😭 No object at position {i}"))
    }

    pub fn set_field(obj: &mut impl Struct, i: usize, prop: Box<dyn Reflect>) {
        get_field_mut(obj, i)
            .set(prop)
            .unwrap_or_else(|error| panic!("😭 Not able to set object at position {i}: {error:?}"));
    }

    pub fn toggle_field_bool(obj: &mut impl Struct, i: usize) -> bool {
        let val = get_field_mut(obj, i)
            .downcast_mut::<bool>()
            .expect("😥 Can't downcast to mut bool");
        *val = !*val;
        *val
    }
    pub fn cast<T: Reflect + Copy>(field: &dyn Reflect) -> T {
        *field
            .downcast_ref::<T>()
            .unwrap_or_else(|| panic!("😥 Can't downcast to {}", field.reflect_type_path()))
    }
}
