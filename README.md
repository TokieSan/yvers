# yvers


The project involves creating a new TUI system monitor and a Linux Process Manager called yvers. This is because the previous system, ytop, is outdated and has hard-coded widgets that cannot be dynamically allocated. Git is used to enable the addition or removal of widgets and users can create their own patches. Our goal is to create a customizable CLI for developers.

<div align="center">
<img src="https://user-images.githubusercontent.com/78858916/236866996-54b446d4-8419-469b-b698-60baa0e73225.gif" alt="1" width="100%">
<img src="https://user-images.githubusercontent.com/78858916/236867126-9db5abfa-5d46-464c-8920-91c8433ec84c.gif" alt="2" width="100%">
<img src="https://user-images.githubusercontent.com/78858916/236867211-cf36c46f-e22f-4334-915c-9299ef6a14f4.gif" alt="3" width="100%">
</div>

## Installation

yvers currently works on Linux and macOS with support planned for all major platforms.

To install yvers, after installing rust, run on the terminal the command:

`$ cargo install yvers`

It will install the latest official published version of the crate. Then, to run it, use the
command:

`$ yvers`


## Usage

### Keybinds

- Quit: `q` or `<C-c>`
- Pause: `<Space>`
- Process navigation:
	- `k` and `<Up>`: up
	- `j` and `<Down>`: down
	- `<C-u>`: half page up
	- `<C-d>`: half page down
	- `<C-b>`: full page up
	- `<C-f>`: full page down
	- `gg` and `<Home>`: jump to top
	- `G` and `<End>`: jump to bottom
- Process actions:
	- `<Tab>`: toggle process grouping
	- `dd`: kill selected process or process group
- Process sorting:
	- `p`: PID/Count
	- `n`: Command
	- `c`: CPU
	- `m`: Mem
- Process filtering:
	- `/`: start editing filter
	- (while editing):
		- `<Enter>`: accept filter
		- `<C-c>` and `<Escape>`: clear filter
- CPU and Mem graph scaling:
	- `h`: scale in
	- `l`: scale out
- `?`: toggles keybind help menu

### Mouse

- click to select process
- mouse wheel to scroll through processes

### How to Install a Patch?

To install a patch, you can clone the git repository then apply the patch from its location.

`$ git clone https://github.com/TokieSan/yvers.git`

`$ cd yvers/`

`$ git apply/path/to/patch/[patch−name].patch`


### CLI Options

```
USAGE:
    yvers [FLAGS] [OPTIONS]

FLAGS:
    -a, --average-cpu    Show average CPU in the CPU widget
    -B, --battery        Show battery widget
    -C, --cpu            Show CPU widget
    -E, --everything     Show all widgets
    -h, --help           Prints help information
    -N, --net            Show Network widget
    -p, --per-cpu        Show each CPU in the CPU widget
    -P, --no-proc        Hide processes widget
    -s, --statusbar      Show a statusbar with the time
    -V, --version        Prints version information

OPTIONS:
    -c, --colorscheme <colorscheme>    Set a colorscheme [default: default]
    -i, --interface <interface>      
    The name of the network interface to show in the Net widget. 'all' shows all interfaces [default: all]
    -I, --interval <interval>          
    Interval in seconds between updates of the CPU and Mem widgets. Can specify either a whole number 
    or a fraction with a numerator of 1 [default: 1]
```
