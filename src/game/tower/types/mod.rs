use super::*;
pub(super) mod gun;
pub(super) mod microwave;
pub(super) mod tesla;

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TowerType {
    Gun,
    Tesla,
    Microwave,
}
