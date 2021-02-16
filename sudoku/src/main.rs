//

mod sudoku;
use sudoku::*;

use std::{thread, env, time::Instant};

use druid::widget::prelude::*;
use druid::{
	commands,   AppDelegate, AppLauncher, Command, Data, DelegateCtx, ExtEventSink, Handled, Lens, LensExt,
	Selector, Target, WidgetExt, WindowDesc, Application, TimerToken, FileDialogOptions, FileSpec, UnitPoint, 
   TextLayout, FontDescriptor,
   { kurbo::{Rect, Line, Ellipse}, widget::prelude::*, {Color, Point}},
	piet::{FontFamily, ImageBuf, ImageFormat, Text, TextLayoutBuilder},
	widget::{Flex, Label, Align, Button, Stepper, TextBox,  LensWrap, Parse, Image, Either,  List, Scroll},
};


const LEVEL :  Level = Level::Easy;

const DISPLAY_SOLUTION: Selector<UI> = Selector::new("display_solution");

macro_rules!  update {
   ($sink : ident, $ui : ident) => {
      $sink.submit_command(DISPLAY_SOLUTION, $ui, Target::Auto).unwrap();
   }; 
   ($sink : ident, $ui : expr) => {
      $sink.submit_command(DISPLAY_SOLUTION, $ui, Target::Auto).unwrap();
   };
}



// UI
#[derive(Clone, Default, Data, Lens)]
struct UI {
	sudoku 	  	  : Sudoku,
	status		  : String,
	running		  : bool,
	max_solutions : usize,
}


impl UI {
   fn new(n : usize) -> Self {
      Self { sudoku : Sudoku::new(n), status : String::new(), running : false, max_solutions : 0}
   }

   fn switch_run(&mut self,  sink: ExtEventSink) {
     
      if self.sudoku.is_running() {  self.sudoku.set_abort(true)  } 
      else {       
         
         let mut _ui = self.clone();
         let solved = _ui.sudoku.is_solved();

         if solved { _ui.status = format!("generating new problem...") } 
         else      { _ui.status = format!("solving...") } 
         
         update!(sink, _ui);
         
         let mut _ui = self.clone();

         thread::spawn(move || {
            if solved {
               _ui.sudoku.gen_problem(LEVEL);
               _ui.status = format!("new problem generated");
            } else {               
               if let Ok(lap) = _ui.sudoku.solve() {  
                  _ui.status = format!("problem solved in {:.0?} with {} evals", lap, num_fmt(_ui.sudoku.get_count_evals()))
               } else {
                  _ui.status = format!("process aborted, new problem generated");
                  _ui.sudoku.gen_problem(LEVEL);
               }
            }
            update!(sink, _ui);
         });
      }
   }

   fn draw(&self, ctx: &mut PaintCtx ) {
      let (sz_box, n) = self.sudoku.get_size();


		let (w, h) = (ctx.size().width, ctx.size().height);
      let w_gt = w > h;

      let (x0, y0, ds, bs) = if w_gt { ( (w - h) / 2., 0., h/n as f64, h ) } else {( 0., (h - w) / 2., w/n as f64, w )};
		
		ctx.fill(Rect::new(0., 0., w, h), &Color::WHITE); // cls

      for i in 0..n+1 {  // lines
         let ids = i as f64 * ds;
         let (width, color) = if i % sz_box == 0 {(2.5, Color::RED)} else { (0.7, Color::BLACK) };
         ctx.stroke( Line::new(Point::new(x0, y0 + ids), Point::new(x0 + bs, y0 + ids)), &color, width);
         ctx.stroke( Line::new(Point::new(x0 + ids, y0), Point::new(x0 + ids, y0 + bs)), &color, width);
      }

      let n = n as usize;
      // let font = env.get(theme::UI_FONT).family;

      for row in 0..n { // board
         for col in 0..n {
            let (b, ch) = self.sudoku.get(row,col);

            if b != 0  {
               let layout = ctx.text()
                  .new_text_layout(format!("{}", ch))
                  .font(FontFamily::MONOSPACE, ds/2.)
                  .text_color(Color::BLUE).build().unwrap();

               ctx.draw_text(&layout, Point::new(x0 + col as f64 * ds + ds/3., y0 + row as f64 * ds + ds/4.) )        
            }
        }
      }
   }

   fn copy_clip(&self) {
      if !self.sudoku.is_running() {
         let mut clipboard = Application::global().clipboard();
         clipboard.put_string(self.sudoku.to_string())
      }
   }
}

impl Widget<UI> for Sudoku {
   fn event(&mut self, ctx: &mut EventCtx, event: &Event, _ui: &mut UI, _env: &Env) {
     match event {
        Event::MouseDown(_me) => { 
         _ui.switch_run( ctx.get_external_handle() );        
         ctx.request_paint();
        }
        Event::Wheel(_me) => { } // _me.wheel_delta.y < 0.0

        Event::Timer(_id) => { }

        Event::WindowConnected => {
            _ui.sudoku.gen_problem(LEVEL);  
            *self = _ui.sudoku.clone();  
           
            _ui.status=format!("sudoku problem generated");
           
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
     _ui.draw(ctx);
  }
}

fn ui_builder() -> impl Widget<UI> {
   Flex::column().
      with_child(Sudoku::default()).
      with_child(
         Align::left( Label::new(|ui: &UI, _: &Env| { ui.status.clone()  } )
            .with_text_size(15.0)).on_click(|_ctx, ui: &mut UI, _env| { ui.copy_clip() }))
}

// command 

struct Delegate;

impl AppDelegate<UI> for Delegate {
    fn command( &mut self,   _ctx: &mut DelegateCtx,   _target: Target,    cmd: &Command,    ui: &mut UI,    _env: &Env,    ) -> Handled {
		
         if let Some(_ui) = cmd.get(DISPLAY_SOLUTION) { 
            *ui = _ui.clone(); // clone result _ui
            Handled::Yes
         } else {
            Handled::No
         }
   } 
}


fn main() {
   fn read_n() -> usize {
      const N : usize = 4;

      let args: Vec<String> = env::args().collect();
      if args.len() > 1 { 
         if let Ok(n) = args[1].parse::<usize>() { n }
         else { N }
      } else { N }
   }

   let n = read_n();

   let main_window = WindowDesc::new(ui_builder)
   .title(format!("Sudoku {}", n))
   .window_size((700., 700.+20.));

   AppLauncher::with_window(main_window)
      .delegate(Delegate {})
      .launch(UI::new(n))
      .expect("launch failed");
}
