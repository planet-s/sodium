#![feature(stmt_expr_attributes)]
#![feature(collections_range)]
#![feature(type_ascription)]

#[cfg(feature = "orbital")]
extern crate orbclient;

extern crate collections;

#[macro_use]
pub mod macros;

pub mod editor;
pub mod buffer;
pub mod parse;
pub mod key_state;
pub mod key;
pub mod prompt;
pub mod open;
pub mod redraw;
pub mod options;
pub mod position;
pub mod graphics;
pub mod selection;
pub mod mode;
pub mod movement;
pub mod motion;
pub mod cursor;
pub mod insert;
pub mod delete;
pub mod exec;
pub mod invert;

pub fn main() {
    use editor::Editor;
    use buffer::SplitBuffer;

    Editor::<SplitBuffer>::init();
}
