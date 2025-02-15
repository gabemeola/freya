pub mod keyboard;
pub mod mouse;
pub mod touch;
pub mod wheel;

use dioxus_core::Event;
pub use keyboard::KeyboardData;
pub use mouse::MouseData;
pub use touch::TouchData;
pub use wheel::WheelData;

pub type KeyboardEvent = Event<KeyboardData>;
pub type MouseEvent = Event<MouseData>;
pub type WheelEvent = Event<WheelData>;
pub type TouchEvent = Event<TouchData>;
