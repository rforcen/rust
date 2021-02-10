/*
	Domain Coloring
*/

#![allow(dead_code)]
#![allow(unused_imports)]


use image::{ImageBuffer, Rgb, Rgba};
use std::{thread, sync::{Arc, Mutex}, convert::TryInto};

use druid::widget::prelude::*;
use druid::im::{vector, Vector};
use druid::{
	commands,   AppDelegate, AppLauncher, Command, Data, DelegateCtx, RenderContext, ExtEventSink, Handled, Lens, LensExt,
	Selector, Target, WidgetExt, WindowDesc, Application, TimerToken, FileDialogOptions, FileSpec, UnitPoint, Rect,
	piet::{ImageBuf, ImageFormat, InterpolationMode},
	widget::{Flex, Label, Align, Button, Stepper, TextBox,  LensWrap, Parse, Image, Either,  List, Scroll},
};

use std::time::{Instant, Duration};
use image;

const WINDOW_SIZE : (f64, f64) = (800., 800.);
const DC_SIZE : u32 = WINDOW_SIZE.0  as u32;
const START_PREDEF_FUNC : f64 = 18.;
	
mod dc;


const PREDEF_FUNCS : [&str; 19] = [
	"acos(c(1,2)*log(sin(z^3-1)/z))", 			"c(1,1)*log(sin(z^3-1)/z)", 					"c(1,1)*sin(z)",
	"z + z^2/sin(z^4-1)", 						"log(sin(z)+1)", 								"cos(z)/(sin(z^4-1))", 
	"z^6-1",									"(z^2-1) * (z-c(2,1))^2 / (z^2+c(2,1))", 		"sin(z)*c(1,2)", 
	"sin(1/z)", "sin(z)*sin(1/z)",				"1/sin(1/sin(z))", "z", "(z^2+1)/(z^2-1)", 		"(z^2+1)/z", "(z+3)*(z+1)^2",
	"(z/2)^2*(z+c(1,2))*(z+c(2,2))/z^3", 		"(z^2)-0.75-c(0,0.2)",							"z * sin( c(1,1)/cos(3/z) + tan(1/z+1) )"];

// UI
#[derive(Clone, Default, Data, Lens)]
struct UI {
	dc			  : dc::DomainColoring,
	expression	  : String,
	stepper_value : f64,
	update		  : bool,
	status		  : String,
}

impl UI {
	fn new(w : u32, h : u32, func : &str) -> Self {
		 Self { 
			dc : dc::DomainColoring::new(w, h, func), 
			expression : func.to_string() , stepper_value : START_PREDEF_FUNC, update:true, status : "".to_string() }
	}
}

impl Widget<UI> for dc::DomainColoring {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _ui: &mut UI, _env: &Env) {
		match event {
			Event::WindowConnected => {
				_ui.dc = self.clone();   	// assign dc to ui
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
		Size { width:bc.max().min_side(), height:bc.max().min_side()}
	}
    fn paint(&mut self, ctx: &mut PaintCtx, _ui: &UI, _env: &Env) {	
		let dc = &_ui.dc;
		if dc.has_image() {
			let image = dc.load_image().to_image(ctx.render_ctx);
			let rect = ctx.size().to_rect();
			ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
		}
	}
}

fn ui_builder() -> impl Widget<UI> {

	let mut dc = dc::DomainColoring::new(DC_SIZE, DC_SIZE, PREDEF_FUNCS[START_PREDEF_FUNC as usize]);
	dc.generate_parallel();

	Flex::column()
		.with_child(
			Flex::row()
				.with_child(LensWrap::new(
					Stepper::new().with_range(0.0, 18.0).with_step(1.0).with_wraparound(true).padding((5.,5.)), UI::stepper_value)
						.on_click(|_ctx, ui: &mut UI, _env| { 
							ui.expression = PREDEF_FUNCS[ui.stepper_value as usize].to_string();
							ui.dc.compile(&ui.expression);						
						}))
				.with_child(Button::new("draw").on_click(|_ctx, ui: &mut UI, _env| { 
					ui.dc.compile(&ui.expression);		
					ui.update =! ui.update; // trigger update
					ui.status = format!("compiled expression: {}", if ui.dc.zvm.ok() {"ok"} else {"error"} );
				}))
				.with_child(Align::left(TextBox::new().fix_width(800.).lens(UI::expression))))
				
		.with_child(dc)
		.with_child(Align::left(Label::new(|ui: &UI, _: &Env| { ui.status.clone() } ).with_text_size(10.0)))
}

fn main() {
	let main_window = WindowDesc::new(ui_builder)
	.title("Domain Coloring")
	.window_size((WINDOW_SIZE.0, WINDOW_SIZE.1+40.));

AppLauncher::with_window(main_window)
	.launch(UI::new(DC_SIZE, DC_SIZE, PREDEF_FUNCS[START_PREDEF_FUNC as usize]))
	.expect("launch failed");
}