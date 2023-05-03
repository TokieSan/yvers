mod battery;
mod block;
mod cpu;
mod help_menu;
mod mem;
mod net;
mod proc;
mod statusbar;

pub use self::battery::BatteryWidget;
pub use self::cpu::CpuWidget;
pub use self::help_menu::HelpMenu;
pub use self::mem::MemWidget;
pub use self::net::NetWidget;
pub use self::proc::ProcWidget;
pub use self::statusbar::Statusbar;
