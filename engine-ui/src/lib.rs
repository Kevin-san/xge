//! engine-ui crate - UI 系统
//!
//! 提供游戏引擎的 UI 系统实现，包括布局、控件、文本渲染等功能。

#![warn(missing_docs)]
#![allow(missing_docs)]

pub mod input;
pub mod layout;
pub mod style;
pub mod text;
pub mod ui_node;
pub mod widgets;

pub use input::{UiEvent, UiEventType, UiInput};
pub use layout::{LayoutDirection, LayoutProperties, LayoutType};
pub use text::{Font, FontSize, TextAlign};
pub use ui_node::{UiNode, UiNodeType, UiRoot};
pub use widgets::{Button, CheckBox, Label, Panel, TextBox};
