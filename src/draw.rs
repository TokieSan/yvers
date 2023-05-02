use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{Frame, Terminal};

use crate::app::{App, Widgets};

// Count number of widgets
// In the future, some generic class "widget" needs to be created
// so a smarter vector of pointers thing can be used instead

pub fn num_active_widgets(widgets: &mut Widgets) -> usize {
    let mut count = 2;
    if widgets.battery.is_some() {
        count += 1;
    }
    if widgets.cpu.is_some() {
        count += 1;
    }
    if widgets.disk.is_some() {
        count += 1;
    }
    if widgets.net.is_some() {
        count += 1;
    }
    if widgets.temp.is_some() {
        count += 1;
    }
    count 
}

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
        .draw(|mut frame| {
            let num_widgets = num_active_widgets(&mut app.widgets);
            let mut chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(100)])
                .split(frame.size());
            if let Some(statusbar) = app.statusbar.as_mut() {
                chunks = Layout::default()
                    .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                    .split(frame.size());
                frame.render_widget(statusbar, chunks[1]);
			}
            draw_widgets(&mut frame, &mut app.widgets, chunks[0], num_widgets);
        })
		.unwrap();
}

// TODO: Make this do it in the form of a grid with smarter layout
pub fn draw_widgets<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect, num_widgets: usize) {
    let constraints = vec![Constraint::Ratio(1, num_widgets as u32); num_widgets];
    let chunks = Layout::default()
        .direction(Direction::Vertical) 
        .constraints(&*constraints)
        .split(area);

    let mut row_idx = 0;
    if let Some(battery) = widgets.battery.as_ref() {
        frame.render_widget(battery, chunks[row_idx]);
        row_idx += 1;
    }
    
    if let Some(disk) = widgets.disk.as_ref() {
        frame.render_widget(disk, chunks[row_idx]);
        row_idx += 1;
    }

    if let Some(cpu) = widgets.cpu.as_ref() {
        frame.render_widget(cpu, chunks[row_idx]);
        row_idx += 1;
    }

    frame.render_widget(&widgets.mem, chunks[row_idx]);
    row_idx += 1; 

    if let Some(temp) = widgets.temp.as_ref() {
        frame.render_widget(temp, chunks[row_idx]);
        row_idx += 1;
    }
    
    if let Some(net) = widgets.net.as_ref() {
        frame.render_widget(net, chunks[row_idx]);
        row_idx += 1;
    }
        
    frame.render_widget(&mut widgets.proc, chunks[row_idx]); 
}

pub fn draw_help_menu<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
		.draw(|mut frame| {
			let rect = app.help_menu.get_rect(frame.size());
			frame.render_widget(&app.help_menu, rect);
		})
		.unwrap();
}

// TODO: figure out how to draw the proc widget without clearing rest of the screen
pub fn draw_proc<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	draw(terminal, app);
	// terminal.draw(|mut frame| {
	// 	let chunks = if app.statusbar.is_some() {
	// 		Layout::default()
	// 			.constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
	// 			.split(frame.size())
	// 	} else {
	// 		Layout::default()
	// 			.constraints(vec![Constraint::Percentage(100)])
	// 			.split(frame.size())
	// 	};

	// 	let vertical_chunks = if app.widgets.temp.is_some() {
	// 		Layout::default()
	// 			.direction(Direction::Vertical)
	// 			.constraints(
	// 				[
	// 					Constraint::Ratio(1, 3),
	// 					Constraint::Ratio(1, 3),
	// 					Constraint::Ratio(1, 3),
	// 				]
	// 				.as_ref(),
	// 			)
	// 			.split(chunks[0])
	// 	} else {
	// 		Layout::default()
	// 			.direction(Direction::Vertical)
	// 			.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
	// 			.split(chunks[0])
	// 	};

	// 	let horizontal_chunks = Layout::default()
	// 		.direction(Direction::Horizontal)
	// 		.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
	// 		.split(*vertical_chunks.last().unwrap());
	// 	app.widgets.proc.render(&mut frame, horizontal_chunks[1]);
	// })
}

// TODO: figure out how to draw the graphs without clearing rest of the screen
pub fn draw_graphs<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	draw(terminal, app);
}
