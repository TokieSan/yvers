use crate::args::Args;
use crate::colorscheme::Colorscheme;
use crate::widgets::*;

pub struct App<'a> {
	pub help_menu: HelpMenu<'a>,
	pub statusbar: Option<Statusbar<'a>>,
	pub widgets: Widgets<'a>,
}

pub struct Widgets<'a> {
	pub battery: Option<BatteryWidget<'a>>,
	pub cpu: Option<CpuWidget<'a>>,
	pub proc: Option<ProcWidget<'a>>,
}

pub fn setup_app<'a, 'b>(
	args: &'b Args,
	colorscheme: &'a Colorscheme,
	program_name: &str,
) -> App<'a> {
	let help_menu = HelpMenu::new(colorscheme);

    let battery = if args.battery || args.everything {
        Some(BatteryWidget::new(colorscheme)) 
    } else {
        None
    };

    let cpu = if args.cpu || args.everything {
        Some(CpuWidget::new(colorscheme, args.interval, args.average_cpu, args.per_cpu))
    } else {
        None
    };  

    let proc = if !args.proc || args.everything {
        Some(ProcWidget::new(colorscheme))
    } else {
        None
    };

	let statusbar = if args.statusbar || args.everything {
		Some(Statusbar::new(colorscheme, program_name))
	} else {
		None
	};

	App {
		help_menu,
		statusbar,
		widgets: Widgets {
			battery,
			cpu,
			proc,
		},
	}
}
