mod disk;
mod temp;
mod net;
mod mem;
mod battery;
/*Add your widget name here*/
/*Add your patch here*/
mod proc;
mod statusbar;
mod block;
mod cpu;
mod help_menu;

pub use self::disk::DiskWidget;
pub use self::temp::TempWidget;
pub use self::mem::MemWidget;
pub use self::battery::BatteryWidget;
/*Add your widget function prototype here*/
/*Add your patch here*/
pub use self::help_menu::HelpMenu;
pub use self::cpu::CpuWidget;
pub use self::net::NetWidget;
pub use self::proc::ProcWidget;
pub use self::statusbar::Statusbar;
