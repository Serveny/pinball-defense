use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct RelEntity(pub Entity);

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
}
