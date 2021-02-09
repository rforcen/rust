// n - queens solver

mod queen;
use queen::*;

use std::{thread, sync::{Arc, Mutex}};

use druid::widget::prelude::*;
use druid::im::{vector, Vector};
use druid::{
	commands,   AppDelegate, AppLauncher, Command, Data, DelegateCtx, ExtEventSink, Handled, Lens, LensExt,
	Selector, Target, WidgetExt, WindowDesc, Application, TimerToken, FileDialogOptions, FileSpec, UnitPoint, 
	piet::{ImageBuf, ImageFormat},
	widget::{Flex, Label, Align, Button, Stepper, TextBox,  LensWrap, Parse, Image, Either,  List, Scroll},
};

use std::time::{Instant, Duration};
use image;

static NQ : i32 = 8; // initial queens
const MIN_QUEENS : f64 = 4.;
const MAX_QUEENS : f64 = 128.;
const LIST_WIDTH : f64 = 200.;
const WINDOW_WIDTH : f64 = 800.;

const COPY_SOLUTION: Selector<UI> = Selector::new("found_solution");
const DISPLAY_SOLUTION: Selector<UI> = Selector::new("display_solution");

static TIMER_INTERVAL: Duration = Duration::from_millis(500);

enum FindType {
	FindFirstFast,
	FindFirst,
	FindAll,
}

// timer
#[derive(Clone, Data)]
struct Timer { 
	lap 		: u128,
	active 		: bool,
	#[data(ignore)] duration 	: Duration,
	#[data(ignore)] now 		: Instant,
	#[data(ignore)] timer_id 	: Arc<Mutex<TimerToken>>,
	#[data(ignore)] running 	: Arc<Mutex<bool>>,	
}

impl Timer { 
	fn new(duration : Duration) -> Self { 
		Self{lap:0, active : false, 
			timer_id : Arc::new(Mutex::new(TimerToken::INVALID)), duration, now : Instant::now(), 
			running : Arc::clone(&Arc::new(Mutex::new(false))) } 
	}

	fn resume(&mut self, ctx: &mut EventCtx) -> &mut Timer { 
		if let Ok(mut t) = self.timer_id.lock() { *t = ctx.request_timer(self.duration) } 
		self.lap = self.now.elapsed().as_millis();
		self
	}
	
	fn start(&mut self, ctx: &mut EventCtx) -> &mut Timer {
		self.lap = 0;
		self.now = Instant::now();
		self.resume(ctx)
	}
	fn enable(&mut self)  { self.lap=0; self.now = Instant::now(); 		self.active=true }
	fn disable(&mut self) { self.lap = self.now.elapsed().as_millis(); 	self.active=false }
	fn is_enabled(&self) -> bool { self.active }

	fn set_running(&mut self, val : bool) { if let Ok(mut lr) = self.running.lock() { *lr = val } 	}

	fn is_running(&self) -> bool 	{ 
		if let Ok(mutex) = self.running.lock() { *mutex } else { false } 
	}
}
impl Default for Timer { fn default() -> Self { Timer::new(TIMER_INTERVAL)  } }

// UI
#[derive(Clone, Default, Data, Lens)]
struct UI {
	queen 	  	  : Queen,
	status		  : String,
	timer		  : Timer,
	n			  : f64,
	running		  : bool,
	max_solutions : usize,
	sol_list	  : Vector<String>,
}

impl 	UI {
	fn new(nq : i32) -> Self 	{

		Self {queen : Queen::new(nq), 
			status:"".to_string(), 
			timer:Timer::new(TIMER_INTERVAL), 
			n:nq as f64, 
			running : false, max_solutions:10, sol_list : vector![]}
	}
	fn start(&mut self) { self.running = true; self.timer.set_running(true);	self.timer.enable();  }
	fn stop(&mut self)  { self.running = false; self.timer.set_running(false);	self.timer.disable(); }

	//fn is_running(&self) -> bool { self.timer.is_running() }
	fn is_idle(&self) -> bool { !self.timer.is_running() }
	
	fn get_lap(&self) -> u128 { self.timer.lap }
	fn bps(&self) -> f64 { (self.queen.get_evals() as f64 / self.get_lap() as f64).floor() }

	fn update_stat(&mut self) { self.status = format!("# queens: {}", self.queen.n ) } 
	fn setn(&mut self) { 
		 self.stop(); 
		 self.queen = Queen::new(self.n as i32);  
		 self.update_stat();
		 self.update_list()
	}

	fn update_list(&mut self) { // from 1..nsols+1
		self.sol_list.clear();
		for (i, s) in self.queen.solutions.iter().enumerate() {
			self.sol_list.push_back(format!("{}: {}", i+1, s.to_string()))
		}
	}

	fn find_solution(&mut self, sink: ExtEventSink, find_type : FindType) { // threaded search -> command

		let mut ui = self.clone();
		
		thread::spawn(move || {		
			match find_type {
				FindType::FindFirst => {
					ui.queen.find_first_solution_mt_control(&ui.timer.running);			
				}
				FindType::FindAll => {
					ui.queen.set_max_solutions(ui.max_solutions as usize);
					ui.queen.find_all_solutions_mt_control(&ui.timer.running);
				}
				FindType::FindFirstFast => {
					ui.queen.find_first_solution();
				}
			}
	
			ui.stop();
			if ui.queen.solutions.len() > 0 { // found a solution -> copy solution to self (not aborted)
				ui.queen.set_solution(0);
				ui.update_list();
				sink.submit_command(COPY_SOLUTION, ui, Target::Auto) // self = ui
					.expect("command failed to submit");
			}
		});
	}

	fn solve(&mut self, find_type : FindType, ctx: &mut EventCtx) {
		if self.is_idle() {
			self.status = match find_type {
				FindType::FindFirst 	=> format!("finding first solution..."),
				FindType::FindFirstFast => format!("finding first solution fast ST no control..."),
				FindType::FindAll 		=> format!("generating all solutions..."),
			};
			self.start();

			self.find_solution(ctx.get_external_handle(),  find_type);
		} else {
			self.stop();
			self.status = format!("scan stopped at lap {} ms, after evaluating {} boards, that's {} boards/sec", 
				self.get_lap(),self.queen.get_evals(), self.bps() )
		}
	}

	fn next(&mut self) {
		if self.is_idle() && self.queen.has_solutions() {
			self.queen.next_solution();
			self.status = format!("solution {} of {}", self.queen.current_sol+1, self.queen.solutions.len())
		}
	}
	fn prev(&mut self) {
		if self.is_idle() && self.queen.has_solutions() {
			self.queen.prev_solution();
			self.status = format!("solution {} of {}", self.queen.current_sol+1, self.queen.solutions.len())
		}
	}

	fn save(&mut self, ctx: &mut EventCtx) {
		let txt = FileSpec::new("Text file", &["txt"]);
		let default_save_name = format!("queen{}.txt", self.queen.n);
		let save_dialog_options = FileDialogOptions::new()
			.allowed_types(vec![txt])
			.default_type(txt)
			.default_name(default_save_name)
			.name_label("Target")
			.title("Choose a target for this file")
			.button_text("Save");

		ctx.submit_command(Command::new(
				druid::commands::SHOW_SAVE_PANEL,
				save_dialog_options.clone(),
				Target::Auto,
		));
	}

	fn get_timer(&self) -> TimerToken { if let Ok(_id) = self.timer.timer_id.lock() {*_id} else { TimerToken::INVALID }}
}

// command 

struct Delegate;

impl AppDelegate<UI> for Delegate {
    fn command( &mut self,   _ctx: &mut DelegateCtx,   _target: Target,    cmd: &Command,    ui: &mut UI,    _env: &Env,    ) -> Handled {
		if let Some(_ui) = cmd.get(COPY_SOLUTION) { // found sol. update ui with _ui
			
			*ui = _ui.clone(); // clone result _ui

			ui.stop();
			ui.status = format!("{} solutions found in {} ms, {} evals", ui.queen.solutions.len(), ui.timer.lap, ui.queen.get_evals());			

            Handled::Yes
        } else {
			if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
				let sols : String = ui.queen.solutions.iter().map(|b| b.to_string()+"\n").collect();
				if let Err(e) = std::fs::write(file_info.path(), sols) {
					println!("Error writing file: {}", e);
				}
				Handled::Yes
			} else {
				if let Some(_ui) = cmd.get(DISPLAY_SOLUTION) { // _ui is '0' size, it's status contains intem selected in list
					fn get_index(s : &String) -> usize { s[..s.find(':').unwrap()].parse::<usize>().unwrap() - 1}

					let msg_index = _ui.status.clone(); // # : solution list

					ui.queen.set_solution(get_index(&msg_index));
					ui.status = msg_index;

					Handled::Yes
				} else {
					Handled::No
				}
			}
		}
        
	}
}


impl Widget<UI> for Queen {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _ui: &mut UI, _env: &Env) {
		match event {
			Event::MouseDown(me) => { // left half -> find first, rigth half -> find all
				if me.pos.x < ctx.size().width / 2.0 { _ui.solve(FindType::FindFirstFast, ctx) }
				else 						         { _ui.solve(FindType::FindAll ,ctx)   }
			}
			Event::Wheel(me) => {
				if me.wheel_delta.y < 0.0 { _ui.next(); /* up */   } 
				else 					  {	_ui.prev(); /* down */ }
			}

			Event::Timer(id) => {
				if *id == _ui.get_timer() {
					if  _ui.timer.is_enabled() { // this comes from cloned ui so _ui.queen is not the real processing instance
						_ui.status = format!("scanning..., lap: {} ms", _ui.get_lap())
					}
					_ui.timer.resume(ctx);
				}
			}

			Event::WindowConnected => {
				_ui.timer.start(ctx); 		// start timer (disabled)
				_ui.queen = self.clone();   // assign queen to ui
				ctx.set_focus(ctx.widget_id());
				ctx.request_focus() 		// support key stroke               
			}

            _ => ()
		}
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _ui: &UI, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &UI, _ui: &UI, _env: &Env) {
		_ctx.request_paint();
	}
    fn layout(&mut self,   _layout_ctx: &mut LayoutCtx,    bc: &BoxConstraints,   _ui: &UI,    _env: &Env,   ) -> Size {	
		let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size { width: min_side, height: min_side }
	}
    fn paint(&mut self, ctx: &mut PaintCtx, _ui: &UI, _env: &Env) {	
		_ui.queen.draw(ctx);
	}
}

fn ui_builder() -> impl Widget<UI> {
	fn load_icon(bytes : &[u8], (w,h) : (u32,u32)) -> ImageBuf {
		let pngd = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
			.unwrap()
			.resize(w, h, image::imageops::FilterType::Triangle)
			.into_rgba8().into_vec();
		ImageBuf::from_raw(pngd, ImageFormat::RgbaSeparate, w as usize, h as usize)
	}

	// tool bar widget

	let icon_size = (20, 20); // icon set
	let first_icon = load_icon(include_bytes!("icons/1st.png"), icon_size);
	let all_icon = load_icon(include_bytes!("icons/all.png"), icon_size);
	let stop_icon = load_icon(include_bytes!("icons/stop.png"), icon_size);
	let stop_icon1 = stop_icon.clone();
	let save_icon = load_icon(include_bytes!("icons/save.png"), icon_size);
	let trans_icon = load_icon(include_bytes!("icons/transformation.png"), icon_size);
	let print_icon = load_icon(include_bytes!("icons/print.png"), icon_size);
	let exit_icon = load_icon(include_bytes!("icons/exit.png"), icon_size);
	
	 // find first group
	 let icon_first = Image::new(first_icon).on_click(|ctx, ui: &mut UI, _env| { ui.solve(FindType::FindFirst, ctx)	});
	 let icon_stop = Image::new(stop_icon).on_click(|ctx, ui: &mut UI, _env|   { ui.solve(FindType::FindFirst, ctx)	});
	 let tb_first_icon = Either::new(|ui, _env|  ui.is_idle(), icon_first, icon_stop );
	 
	 // find all group
	 let icon_all = Image::new(all_icon).on_click(|ctx, ui: &mut UI, _env| { ui.solve(FindType::FindAll, ctx)	});
	 let icon_stop_all = Image::new(stop_icon1).on_click(|ctx, ui: &mut UI, _env|   { ui.solve(FindType::FindAll, ctx)	});
	 let tb_all_icon = Either::new(|ui, _env|  ui.is_idle(), icon_all, icon_stop_all );
  	
	let max_solutions = LensWrap::new(
		Parse::new(TextBox::new().fix_width(50.0)),
		UI::max_solutions.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0)),
	);

	let tb_transform = Image::new(trans_icon).on_click(|_ctx, ui: &mut UI, _env| {		
		if ui.is_idle() && ui.queen.has_solutions() {
			ui.queen.transformations();
			ui.queen.set_solution(0);
			ui.update_list();

			ui.status = format!("generated {} transformations", ui.queen.solutions.len())
		}
	});
	
	let tb_print = Image::new(print_icon).on_click(|_ctx, ui: &mut UI, _env| {		
		if ui.is_idle() && ui.queen.has_solutions() {
			ui.queen.print_solutions();
		}
	});
	
	
	let n_queens_tb = LensWrap::new(
        Parse::new(TextBox::new().fix_width(40.)),
        UI::n.map(|x| Some(*x), |x, y| *x = y.unwrap_or(MIN_QUEENS)),
    ).on_click(|_ctx, ui: &mut UI, _env| { ui.setn() });

	let tb_stepper = Stepper::new()
		.with_range(MIN_QUEENS, MAX_QUEENS)		.with_step(1.0)
		.with_wraparound(true)					.lens(UI::n)
		
		.on_click(|_ctx, ui: &mut UI, _env| { ui.setn() });	

	let tb_save = Image::new(save_icon).on_click(|_ctx, ui: &mut UI, _env| {		
		ui.save(_ctx)
	});
	let tb_exit = Image::new(exit_icon).on_click(|_ctx, ui: &mut UI, _env| {		
		ui.stop();
		Application::global().quit()
	});
	let tool_bar = Align::left(Flex::row() 
					.with_child(tb_first_icon.padding((5.,5.)))
					.with_child(tb_all_icon.padding((5.,5.)))
					.with_child(tb_transform.padding((5.,5.)))
					.with_child(tb_print.padding((5.,5.)))		
					.with_child(tb_save.padding((5.,5.)))
					.with_child(Label::new("# queens:")).with_child(n_queens_tb).with_child(tb_stepper)
					.with_child(Label::new("max solutions: ")).with_child(max_solutions)
					.with_child(tb_exit.padding((50.0,5.0))).fix_width(830.));


	// board
	let board = Queen::new(NQ); // assigned to ui in queen.Event::WindowConnected

	// solutions list

	let w_list = List::new(|| { 
		Button::new(|item: &String, _env: &_| item.clone())
			.fix_width(LIST_WIDTH-20.)
			.on_click(|_ctx, s, _env| { // s in the format # index : solution list
				let mut ui = UI::new(0); // dirty trick: create an empty UI, set status with text of selected item and submit command DISPLAY_SOLUTION
				ui.status = s.clone();
				_ctx.get_external_handle()
					.submit_command(DISPLAY_SOLUTION, ui, Target::Auto) 
					.expect("command failed to submit");
			})});

	let s_list = Scroll::new(w_list).vertical().lens(UI::sol_list).fix_width(LIST_WIDTH).padding((1.,3.))
				.on_click(|_ctx, _ui: &mut UI, _env| {});

	// status
	let status = Align::left( Label::new(|ui: &UI, _: &Env| {
		 	ui.status.clone()
	} ).with_text_size(10.0)).on_click(|_ctx, ui: &mut UI, _env| { ui.queen.test01() });


	Flex::column()
		.with_child(tool_bar)
		.with_flex_child(
			Flex::row()
				.with_child(Align::vertical(UnitPoint::TOP_LEFT, s_list))
				.with_child(board), 1.0)
		.with_child(status)
		
}

fn main() {

    let main_window = WindowDesc::new(ui_builder)
		.title("Queens")
		.window_size((WINDOW_WIDTH + LIST_WIDTH, WINDOW_WIDTH));

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(UI::new(NQ))
		.expect("launch failed");
}
