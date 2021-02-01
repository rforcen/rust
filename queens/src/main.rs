// queens problem

/*
	action keys:

	Esc, q 	-> quit
	1		-> find first (single thread - st)
	m		-> find first (multiple thread - mt)
	z		-> find first (mt) non blocking but with no graph display
	t		-> apply transformations (translate(h,v), rotate90, mirror(h,v))
	a		-> find all solutions (this may take a long time for big n)
	p		-> print solutions
	space	-> random solution
*/

use druid::widget::prelude::*;
use druid::{AppLauncher, Application, Code, WindowDesc};
use std::time::{Instant};

use std::thread;

mod queen;
use queen::*;


impl Widget<()> for Queen {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        match event {
            Event::KeyDown(kevent) => {
				match kevent.code  {
					Code::Escape | Code::KeyQ => {
						println!("exit");
						ctx.window().close();
						Application::global().quit()
					}
					Code::Digit1 => { // first solution ST
						let t = Instant::now();
						
						self.find_first_solution();
						
						println!("first solution ST, lap: {:?}, # evaluated boards: {}", Instant::now()-t, self.count_evals);

						self.set_solution(0);
						ctx.request_paint()
					}
					Code::KeyM => { // first solution MT
						let t = Instant::now();
						
						self.find_first_solution_mt();
						
						println!("first solution MT, lap: {:?}, # evaluated boards: {}", Instant::now()-t, self.count_evals);

						self.set_solution(0);
						ctx.request_paint()
					}
					Code::KeyT => { // apply transformations over current board
						if ! self.solutions.is_empty() {
							self.transformations();

							self.set_random_solution();
							
							ctx.request_paint()
						}
					}
					Code::KeyA => { // All solutions
						self.find_solutions();
						self.set_solution(0);
						ctx.request_paint()		
					}
					Code::KeyP => { // print sols.
						self.print_solutions()
					}
					Code::Space => { // random solution
						if ! self.solutions.is_empty() {
							self.set_random_solution();
							ctx.request_paint()
						} else {
							println!("no solutions to display")
						}
					}
					Code::KeyZ => {
						let rx = self.clone().ff_thread(); // find first mutithread non blocking
	
						thread::spawn(move || { // wait until recv in a non blocking thread
							
							let t = Instant::now();
							let mut q = rx.recv().unwrap(); // get <Queen>  w/solution
							
							q.set_solution(0);
							println!("found solution lap: {:?}, {}\n{}", Instant::now()-t, q.board.to_string(), q.board.draw_string());
							
							// can't display as none 'self' nor ctx is available inside a thread...							
						});	
					}
					_ => ()
					
				}
			}
			Event::MouseDown(_) => {
				ctx.request_paint()				
			}			
			Event::WindowConnected => {
				ctx.request_focus() // support key stroke               
			}
			
            _ => ()
		}
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}
    fn layout(&mut self,   _layout_ctx: &mut LayoutCtx,    bc: &BoxConstraints,   _data: &(),    _env: &Env,   ) -> Size {	
		bc.constrain((0.0, 0.0))    
	}
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {	self.draw(ctx);	}
}

use std::env;

pub fn main() {
	let n_queens = if let Some(nq_str) = env::args().nth(1) { nq_str.parse::<i32>().unwrap() } else {  13  /* default */ };
	
	let queen = Queen::new(n_queens);
	let window = WindowDesc::new(|| queen)
		.window_size((800., 800.))
		.title(format!("Queens {}", n_queens));
    AppLauncher::with_window(window)
        .launch(())
        .expect("launch failed");
}
