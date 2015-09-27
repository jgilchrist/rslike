//! The roguelike's user interface

pub use self::colors::{Color, Colors};
pub use self::console::{Console, Key};
pub use self::gui::GUI;
pub use self::menu::{Menu, MenuOption};
pub use self::widget::Widget;

pub mod screens;
pub mod chars;
pub mod primitives;

mod colors;
mod console;
mod gui;
mod menu;
mod widget;
