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
    pub mem: Option<MemWidget<'a>>,
    pub net: Option<NetWidget<'a, 'b>>,
    pub proc: Option<ProcWidget<'a>>,
}

pub fn setup_app<'a, 'b>(
    args: &'b Args,
    colorscheme: &'a Colorscheme,
    program_name: &str,
) -> App<'a, 'b> {
    let help_menu = HelpMenu::new(colorscheme);

    let battery = if args.battery || args.everything {
        Some(BatteryWidget::new(colorscheme))
    } else {
        None
    };

    let cpu = if args.cpu || args.everything {
        Some(CpuWidget::new(
            colorscheme,
            args.interval,
            args.average_cpu,
            args.per_cpu,
        ))
    } else {
        None
    };

    let mem = if args.mem || args.everything {
        Some(MemWidget::new(colorscheme, args.interval)) 
    } else {
        None
    };

    let net = if args.net || args.everything {
        Some(NetWidget::new(colorscheme, &args.interface))
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
            mem,
            net,
            proc,
        },
    }
}
