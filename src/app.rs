use crate::args::Args;
use crate::colorscheme::Colorscheme;
use crate::widgets::*;

pub struct App<'a, 'b> {
    pub help_menu: HelpMenu<'a>,
    pub statusbar: Option<Statusbar<'a>>,
    pub widgets: Widgets<'a, 'b>,
}

pub struct Widgets<'a, 'b> {
    /*Widget Added for Patch*/
    /*add your patch element here*/
    pub net: Option<NetWidget<'a, 'b>>,
    pub proc: Option<ProcWidget<'a>>,
    pub cpu: Option<CpuWidget<'a>>,
}

pub fn setup_app<'a, 'b>(
    args: &'b Args,
    colorscheme: &'a Colorscheme,
    program_name: &str,
) -> App<'a, 'b> {
    let help_menu = HelpMenu::new(colorscheme);

    /*add function for patch here.*/
    /*add your patch here.*/

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
            /* add var for patch*/
            /* add your patch here*/
            cpu,
            net,
            proc,
        },
    }
}
