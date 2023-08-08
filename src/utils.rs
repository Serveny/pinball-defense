use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct RelParent(pub Entity);
