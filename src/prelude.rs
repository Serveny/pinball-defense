pub use crate::assets::*;
pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_rapier3d::prelude::*;
pub(crate) use macros::*;

pub fn spatial_from_pos(pos: Vec3) -> SpatialBundle {
    SpatialBundle::from_transform(Transform::from_translation(pos))
}

mod macros {

    // Shorthand oneliner for if true give back c
    macro_rules! if_true {
        ($a:expr,$c:expr) => {{
            match $a {
                true => Some($c),
                false => None,
            }
        }};
    }

    // My own little logging feature
    macro_rules! log {
        ($($arg:tt)*) => {{
            #[cfg(feature = "log")]
            println!($($arg)*)
        }};
    }

    pub(crate) use if_true;
    pub(crate) use log;
}
