pub use crate::assets::*;
pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_rapier3d::prelude::*;
pub(crate) use macros::*;

// Shorthand oneliner for if true give back c
mod macros {

    macro_rules! if_true {
        ($a:expr,$c:expr) => {{
            match $a {
                true => Some($c),
                false => None,
            }
        }};
    }

    pub(crate) use if_true;
}
