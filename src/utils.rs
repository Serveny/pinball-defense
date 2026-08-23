use crate::prelude::*;
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::reflect::ReflectMapEntities;

#[derive(Component, Clone, Deref, DerefMut, Reflect)]
#[reflect(Component, MapEntities)]
pub struct RelEntity(#[entities] pub Entity);

impl Default for RelEntity {
    fn default() -> Self {
        RelEntity(Entity::PLACEHOLDER)
    }
}

impl MapEntities for RelEntity {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = mapper.get_mapped(self.0);
    }
}

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
    use bevy::reflect::{Reflect, structs::Struct};

    pub fn prop_name(obj: &impl Struct, i: usize) -> String {
        obj.name_at(i)
            .unwrap_or_else(|| panic!("😭 No name at index {i}"))
            .to_string()
    }

    pub fn get_field_mut(obj: &mut impl Struct, i: usize) -> &mut dyn Reflect {
        obj.field_at_mut(i)
            .unwrap_or_else(|| panic!("😭 No object at position {i}"))
            .try_as_reflect_mut()
            .unwrap_or_else(|| panic!("😭 Can't convert partial to reflect"))
    }

    pub fn set_field(obj: &mut impl Struct, i: usize, prop: Box<dyn Reflect>) {
        get_field_mut(obj, i)
            .set(prop)
            .unwrap_or_else(|error| panic!("😭 Not able to set object at position {i}: {error:?}"));
    }

    pub fn cast<T: Reflect + Copy>(field: &dyn Reflect) -> T {
        *field
            .downcast_ref::<T>()
            .unwrap_or_else(|| panic!("😥 Can't downcast to {}", field.reflect_type_path()))
    }
}
