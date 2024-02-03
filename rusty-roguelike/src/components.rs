pub use crate::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Player;
