// queens

use std::{thread, sync::{Arc, Mutex}};

use druid::widget::prelude::*;

use druid::{
	commands,   AppDelegate, AppLauncher, Command, Data, DelegateCtx, ExtEventSink, Handled, Lens, LensExt,
	Selector, Target, WidgetExt, WindowDesc, Application, TimerToken, FileDialogOptions, FileSpec,
	widget::{Flex, Label, Align, Button, Stepper, TextBox,  LensWrap, Parse},
};

use std::time::{Instant, Duration};

mod queen;
use queen::*;

static NQ : i32 = 8; // initial queens
const MIN_QUEENS : i32 = 4;
const MAX_QUEENS : i32 = 128;

const COPY_SOLUTION: Selector<UI> = Selector::new("found_solution");
static TIMER_INTERVAL: Duration = Duration::from_millis(500);

enum FindType {
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
	file_name	  : String,
	running		  : bool,
	max_solutions : String,
}

impl 	UI {
	fn new(nq : i32) -> Self 	{
		Self {queen : Queen::new(nq), 
			status:"".to_string(), 
			timer:Timer::new(TIMER_INTERVAL), 
			n:nq as f64, file_name : format!("queen{}.txt", nq), 
			running : false, max_solutions:"10".to_string()}
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
		 self.file_name = format!("queen{}.txt", self.queen.n);
		 self.update_stat() 
	}

	fn get_max_solutions(&self) -> usize { if let Ok(ms) = self.max_solutions.parse::<usize>() { ms } else { 0 } }

	fn find_solution(&mut self, sink: ExtEventSink, find_type : FindType) { // threaded search -> command

		let mut ui = self.clone();

		//println!("{:p}, {:p}", ui.timer.running, self.timer.running);
		
		thread::spawn(move || {		
			match find_type {
				FindType::FindFirst => {
					ui.queen.find_first_solution_mt_control(&ui.timer.running);			
				}
				FindType::FindAll => {
					ui.queen.set_max_solutions(ui.get_max_solutions());
					ui.queen.find_all_solutions_mt_control(&ui.timer.running);
				}
			}
	
			ui.stop();
			if ui.queen.solutions.len() > 0 { // found a solution -> copy solution to self (not aborted)
				ui.queen.set_solution(0);
				sink.submit_command(COPY_SOLUTION, ui, Target::Auto) // self = ui
					.expect("command failed to submit");
			}
		});
	}

	fn find_first(&mut self, ctx: &mut EventCtx) {
		if self.is_idle() {

			self.status = format!("finding first solution...");	
			self.start();

			self.find_solution(ctx.get_external_handle(),  FindType::FindFirst);
		} else {
			self.stop();
			self.status = format!("scan stopped at lap {} ms, after evaluating {} boards, that's {} boards/sec", 
				self.get_lap(),self.queen.get_evals(), self.bps() )
		}
	}

	fn find_all(&mut self, ctx: &mut EventCtx){
		if self.is_idle() {
			self.status = format!("generating all solutions...");	
			self.start();

			self.find_solution(ctx.get_external_handle(), FindType::FindAll);
		} else {
			self.stop();
			self.status = format!("scan stopped at lap {} ms, 	after evaluating {} boards, that's {} boards/sec", 
				self.get_lap(),self.queen.get_evals(), self.bps() 	)
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
				Handled::No
			}
		}
        
	}
}


impl Widget<UI> for Queen {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _ui: &mut UI, _env: &Env) {
		match event {
			Event::MouseDown(me) => { // left half -> find first, rigth half -> find all
				if me.pos.x < ctx.size().width / 2.0 { _ui.find_first(ctx) }
				else 						         { _ui.find_all(ctx)   }
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
				_ui.queen = self.clone(); // assign queen to ui
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
	let board = Queen::new(NQ).center(); // assigned to ui in queen.Event::WindowConnected
		
	let status = Align::left( Label::new(|ui: &UI, _: &Env| {
		format!("{}", ui.status)
	 } ).with_text_size(10.0)).on_click(|_ctx, ui: &mut UI, _env| { ui.queen.test01() });

	let tb_first = Button::new(|ui : &UI, _ :&Env|
		match ui.running {
			false => "first".into(),
			true  => "stop".into(),
		})
		.on_click(|ctx, ui: &mut UI, _env| { ui.find_first(ctx)	}).padding((5.,5.));

	let tb_all = Button::new(|ui : &UI, _ :&Env|
		match ui.running {
			false => "all".into(),
			true  => "stop".into(),
		}).on_click(|ctx, ui: &mut UI, _env| { ui.find_all(ctx) })	;
	let max_solutions = TextBox::new().with_placeholder("max solutions").fix_width(50.).lens(UI::max_solutions);

	let tb_transform = Button::new("tranformations").on_click(|_ctx, ui: &mut UI, _env| {		
		if ui.is_idle() && ui.queen.has_solutions() {
			ui.queen.transformations();
			ui.queen.set_random_solution();

			ui.status = format!("generated {} transformations", ui.queen.solutions.len())
		}
	});
	
	let tb_print = Button::new("print").on_click(|_ctx, ui: &mut UI, _env| {		
		if ui.is_idle() && ui.queen.has_solutions() {
			ui.queen.print_solutions();
		}
	});
	
		
	let n_queens = LensWrap::new(
        Parse::new(TextBox::new()).fix_width(40.),
        UI::n.map(|x| Some(*x), |x, y| *x = y.unwrap_or(MIN_QUEENS as f64)),
    ).on_click(|_ctx, ui: &mut UI, _env| { ui.setn() });

	let tb_stepper = Stepper::new()
		.with_range(MIN_QUEENS as f64, MAX_QUEENS as f64)		.with_step(1.0)
		.with_wraparound(true)									.lens(UI::n)
		.padding((5.,5.))
		.on_click(|_ctx, ui: &mut UI, _env| { ui.setn() });	

	let tb_save = Button::new("save").on_click(|_ctx, ui: &mut UI, _env| {		
		ui.save(_ctx)
	});
	let tb_quit = Button::new("quit").on_click(|_ctx, ui: &mut UI, _env| {		
		ui.stop();
		Application::global().quit()
	});
			
	Flex::column()
		.with_child(Align::left(Flex::row() // tool bar
			.with_child(Label::new("# queens:")).with_child(n_queens).with_child(tb_stepper)
			.with_child(tb_first)
			.with_child(tb_all)
			.with_child(Label::new("max solutions: ")).with_child(max_solutions)
			.with_child(tb_transform)
			.with_child(tb_print)		
			.with_child(tb_save)
			.with_child(tb_quit)))
		.with_child(board)
		.with_child(status) 
}

fn main() {

    let main_window = WindowDesc::new(ui_builder).title("Queens").window_size((800., 850.));
    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(UI::new(NQ))
		.expect("launch failed");
}
