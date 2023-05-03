use num_rational::Ratio;
use structopt::StructOpt;

use crate::colorscheme::Colorschemes;

#[derive(StructOpt)]
pub struct Args {
	/// Show average CPU in the CPU widget.
	#[structopt(short = "a", long = "average-cpu")]
	pub average_cpu: bool,

	/// Show widgets or not
    #[structopt(short = "B", long = "battery")]  
	pub battery: bool,   

	#[structopt(short = "C", long = "cpu")]
	pub cpu: bool,  

	#[structopt(short = "N", long = "net")]
	pub net: bool,   

	#[structopt(short = "P", long = "no-proc")]
	pub proc: bool,

	#[structopt(short = "E", long = "everything")]
	pub everything: bool,

    /// Set a colorscheme.
	#[structopt(
		short = "c",
		long = "colorscheme",
		default_value = "default",
		long_help = r"Colorschemes:
    - default
    - default-dark (for white backgrounds)
    - solarized-dark
    - monokai
    - vice
"
	)]
	pub colorscheme: Colorschemes,


	/// The name of the network interface to show in the Net widget. 'all' shows all interfaces.
	#[structopt(short = "i", long = "interface", default_value = "all")]
	pub interface: String,

	/// Interval in seconds between updates of the CPU and Mem widgets. Can specify either a whole number or a fraction with a numerator of 1.
	#[structopt(short = "I", long = "interval", default_value = "1")]
	pub interval: Ratio<u64>,

	/// Only show the CPU, Mem, and Process widgets.
	#[structopt(short = "m", long = "minimal")]
	pub minimal: bool,

	/// Show each CPU in the CPU widget.
	#[structopt(short = "p", long = "per-cpu")]
	pub per_cpu: bool,

	/// Show a statusbar with the time.
	#[structopt(short = "s", long = "statusbar")]
	pub statusbar: bool,
}
