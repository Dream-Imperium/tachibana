#![feature(duration_zero)]
#![feature(duration_constants)]

pub mod framework;
pub mod game;
pub mod utils;

pub use skulpin_renderer::skia_safe as skia;

pub mod prelude {
    pub use crate::framework::{
        music::Music,
        widgets::{LayoutDimension, LayoutSize, Widget, Wrap, WrapState, Wrappable},
    };
    pub use crate::game;
    pub use crate::utils::*;
    pub use skulpin_renderer::skia_safe as skia;
}
