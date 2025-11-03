//! Shadcn-inspired reusable primitives implemented with Dioxus 0.7 signals and the shared `shadcn.css`.
//! Each component mirrors the styling and API conventions of the upstream React components while
//! remaining idiomatic to Rust and Dioxus.

mod accordion;
mod alert;
mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod dropdown_menu;
mod input;
mod label;
mod progress;
mod radio_group;
mod select;
mod separator;
mod slider;
mod switch;
mod tabs;
mod textarea;
mod tooltip;

pub use accordion::*;
pub use alert::*;
pub use avatar::*;
pub use badge::*;
pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use dropdown_menu::*;
pub use input::*;
pub use label::*;
pub use progress::*;
pub use radio_group::*;
pub use select::*;
pub use separator::*;
pub use slider::*;
pub use switch::*;
pub use tabs::*;
pub use textarea::*;
pub use tooltip::*;
