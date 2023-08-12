use super::*;
pub(super) mod gun;
pub(super) mod microwave;
pub(super) mod tesla;

#[derive(Component, Clone, Copy, Debug)]
pub enum TowerType {
    Gun,
    Tesla,
    Microwave,
}
