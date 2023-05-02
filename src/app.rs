use crate::args::Args;
use crate::colorscheme::Colorscheme;
use crate::widgets::*;

pub struct App<'a, 'b> {
	pub help_menu: HelpMenu<'a>,
	pub statusbar: Option<Statusbar<'a>>,
	pub widgets: Widgets<'a, 'b>,
}

pub struct Widgets<'a, 'b> {
	pub battery: Option<BatteryWidget<'a>>,
	pub cpu: Option<CpuWidget<'a>>,
	pub disk: Option<DiskWidget<'a>>,
	pub mem: Option<MemWidget<'a>>,
	pub net: Option<NetWidget<'a, 'b>>,
	pub proc: Option<ProcWidget<'a>>,
	pub temp: Option<TempWidget<'a>>,
}

pub fn setup_app<'a, 'b>(
	args: &'b Args,
	colorscheme: &'a Colorscheme,
	program_name: &str,
) -> App<'a, 'b> {
	let proc = Some(ProcWidget::new(colorscheme));
	let help_menu = HelpMenu::new(colorscheme);

	let (battery, disk, net, temp, cpu, mem) = if args.minimal {
		(None, None, None, None, None, None)
	} else {
		(
			if args.battery {
				Some(BatteryWidget::new(colorscheme))
			} else {
				None
			},
			Some(DiskWidget::new(colorscheme)),
			Some(NetWidget::new(colorscheme, &args.interface)),
			Some(TempWidget::new(colorscheme, args.fahrenheit)),
            Some(CpuWidget::new(colorscheme, args.interval, args.average_cpu, args.per_cpu)),
            Some(MemWidget::new(colorscheme, args.interval)),
		)
	};

	let statusbar = if args.statusbar {
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
			disk,
			mem,
			net,
			proc,
			temp,
		},
	}
}
