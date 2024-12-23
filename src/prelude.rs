pub use crate::assets::*;
pub use avian2d::prelude::*;
pub use bevy::prelude::*;
pub(crate) use macros::*;

pub fn spatial_from_pos(pos: Vec3) -> impl Bundle {
    (Transform::from_translation(pos), Visibility::default())
}

mod macros {

    // My own little logging feature
    macro_rules! log {
        ($($arg:tt)*) => {{
            #[cfg(feature = "log")]
            println!($($arg)*)
        }};
    }

    pub(crate) use log;
}
