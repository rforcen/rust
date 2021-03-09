// radial spectrum UI

mod signal;
use signal::*;
use std::f64::consts::PI;
use itertools::*;


use druid::{
	commands,   AppDelegate, AppLauncher, Command, Data, DelegateCtx, ExtEventSink, Handled, Lens, LensExt,
	Selector, Target, WidgetExt, WindowDesc, Application, TimerToken, FileDialogOptions, FileSpec, UnitPoint, 
   TextLayout, FontDescriptor,
   { kurbo::{Rect, Line, Ellipse}, widget::prelude::*, {Color, Point}},
	piet::{FontFamily, ImageBuf, ImageFormat, Text, TextLayoutBuilder},
	widget::{Flex, Label, Align, Button, Stepper, TextBox,  LensWrap, Parse, Image, Either,  List, Scroll},
};

#[derive(Clone, Default, Data, Lens)]
struct Wave {
#[data(ignore)] spectrum       : (Vec<f32>,Vec<f32>),
#[data(ignore)] musical_matrix : Vec<Vec<f32>>,
                spec           : String,
}

// UI
#[derive(Clone, Default, Data, Lens)]
struct UI {
    wave    : Wave,
}

impl UI {
    fn new() -> Self {
       Self { wave : Wave{ spectrum:(vec![],vec![]), musical_matrix: vec![], spec:String::default()} }
    }

    fn draw(&self, ctx: &mut PaintCtx ) {
        self.draw_spectrum(ctx);
        self.draw_musical_matrix(ctx);
    }

    fn draw_musical_matrix(&self,  ctx: &mut PaintCtx ) {
        let create_layout = |ctx: &mut PaintCtx, val : &str, fs, color| { ctx.text()
            .new_text_layout(format!("{}",val))
            .font(FontFamily::MONOSPACE, fs)
            .text_color(color).build().unwrap() };

        let (w, h) = (ctx.size().width, ctx.size().height);
        let w = w.min(h) / 47.;
        ctx.fill(Rect::new(0., 0., w*13., w*7.), &Color::WHITE); // cls
        let notes : Vec<&str> = "C,C#,D,D#,E,F,F#,G,G#,A,A#,B".split(',').collect::<Vec<&str>>();        

        for n in 0..12 { // labels ( notes, octaves )
            let lo = create_layout(ctx, notes[n], 8., Color::RED);
            ctx.draw_text(&lo, (n as f64 * w + w/6. + w, w/4.));
        }
        for o in -2..=3 {
            let lo = create_layout(ctx, &format!("{:2}",o)[..], 8., Color::RED);
            ctx.draw_text(&lo, (w/6., (o+2) as f64 * w + w + w/3.));
        }
        for o in -2..=3_i32 { // data in circles
            for n in 0..12 {
                let x = self.wave.musical_matrix[(o+2) as usize][n] * 100.;

                if x > 5.0 {
                    let (cx, cy, rad) = ( (n+1) as f64 * w + w/2., (o+2+1) as f64 * w + w/2., w * (x as f64) / 270.);
                    ctx.fill(Ellipse::new((cx,cy), (rad, rad), 0.), if x==100.0 {&Color::RED } else {&Color::BLUE});
                }
            }
        }
        for y in 0..=7 { // grid
            for x in 0..=13 {
                let (px, py) = (x as f64 *w, y as f64 *w);
                ctx.stroke( Line::new((0., py), (w*13., py)), &Color::GREEN, 0.3 );  
                ctx.stroke( Line::new((px, 0.), (px, w*7.)), &Color::GREEN, 0.3 );  
            }
        }
    }

    fn draw_spectrum(&self, ctx: &mut PaintCtx ) {
  
        let create_layout = |ctx: &mut PaintCtx, val : f32, fs, color| { ctx.text()
            .new_text_layout(format!("{:.0}", val))
            .font(FontFamily::MONOSPACE, fs)
            .text_color(color).build().unwrap() };

        let (w, h) = (ctx.size().width, ctx.size().height);
        let (x0, y0) = ( w/2., h/2.);
        let r = if w > h  {h/2.} else {w/2.};
        let mf = 0.9;
        let ( r0, r1, r_txt, r_gr, ri, txt_size ) = (0.91*mf, 0.93*mf, 0.88*mf, 0.89*mf, 0.01, 7.);
          
        ctx.fill(Rect::new(0., 0., w, h), &Color::WHITE); // cls

        ctx.stroke(Ellipse::new((x0, y0), (x0*r1, y0*r1), 0.), &Color::RED, 0.5); // 2 concentric circles
        ctx.stroke(Ellipse::new((x0, y0), (x0*r0, y0*r0), 0.), &Color::BLUE, 0.1);

        let n_ticks = 60;
        for m in 0..n_ticks { // ticks
           
            let alpha = (360./n_ticks as f64) * PI * m as f64 / 180.;

            ctx.stroke( Line::new(
                (x0+r*r0*alpha.cos(), y0+r*r0*alpha.sin()), 
                (x0+r*r1*alpha.cos(), y0+r*r1*alpha.sin())
            ), &Color::GREEN, 0.6 );  

            // label pos. + offset table
            let lo = vec![(0., -0.5), (0.,0.), (-0.5, 0.), (-1.,0.), (-1., -0.5), (-1., -1.), (-0.5, -1.), (0., -0.5)]
                .iter().map(|x|(x.0 * txt_size, x.1 * txt_size)).collect::<Vec<(f64,f64)>>() [8 * m / n_ticks];


            let layout = create_layout(ctx, self.wave.spectrum.0[m*self.wave.spectrum.0.len()/n_ticks], 7., Color::BLUE);

            ctx.draw_text(&layout, (x0+r*r_txt*alpha.cos()+lo.0, y0+r*r_txt*alpha.sin()+lo.1) )         
        }

       
        // draw the spectrum
        let (min_x, max_x) = ( self.wave.spectrum.0[0], self.wave.spectrum.0.last().unwrap() );
        let min_y = self.wave.spectrum.1.iter().fold( f32::MAX, |min,x| min.min(*x));
        let (ind_maxy, max_y) = self.wave.spectrum.1.iter().enumerate()
            .fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
        let max_freq = self.wave.spectrum.0[ind_maxy];

        let lout = ctx.text()
            .new_text_layout(format!("{}", self.wave.spec.clone() + &format!("\nmax freq.: {:.1} hz", max_freq)[..]))
            .font(FontFamily::MONOSPACE, 8.0)
            .text_color(Color::BLUE).build().unwrap();
        ctx.draw_text(&lout, (10., h-60.) );

   
        let to_coord = |px, py| { // x, y coord in spectrum to plt coord
            let x = ((px-min_x)/(max_x-min_x)) as f64; // scaled to [0..1]
            let y = ((py-min_y)/(max_y-min_y)) as f64; 

            let alpha = 2. * PI * x;
            (x0+y*r*r_gr*alpha.cos(), y0+y*r*r_gr*alpha.sin())
        };

        zip(&self.wave.spectrum.0, &self.wave.spectrum.1)
            .map(|(px, py)| { // points outline
                to_coord(px, py)
            }).collect::<Vec<(_,_)>>()
            .windows(2).for_each(|p| { // draw lines & ribbon
                ctx.stroke( Line::new( p[0], p[1] ), &Color::RED, 0.3 );
                p.iter().for_each(|p| ctx.stroke( Line::new( *p,  (x0, y0) ), &Color::GREEN, 0.3 ));  
            });       

        ctx.stroke( Line::new( (x0,y0), to_coord(&max_freq, &max_y )), &Color::RED, 0.7 ); // max freq.
        let layout = create_layout(ctx, max_freq, 9., Color::RED );
        ctx.draw_text(&layout, to_coord(&max_freq, &max_y ) );

        ctx.fill(Ellipse::new((x0, y0), (x0*ri, y0*ri), 0.), &Color::WHITE); // inner circle
     }    

}


impl Widget<UI> for Wave {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _ui: &mut UI, _env: &Env) {
      match event {
         Event::MouseDown(_me) => {  ctx.request_paint();  }
         Event::Wheel(_me) => { } // _me.wheel_delta.y < 0.0
         Event::Timer(_id) => { }
         Event::WindowConnected => {
             _ui.wave = self.clone();  // here we set current wave
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

 use std::env::args;

fn ui_builder() -> impl Widget<UI> {
    if let Some(file_name) = args().nth(1) {  // "/Users/asd/Documents/dev/_voicesync/wav/02 - aaaaaaa.wav";

        let mut signal = Signal::new();
        signal.set_top_freq(600.);
        signal.read_wav(&file_name.to_string());
    
        Flex::column().with_child( Wave { 
                spectrum        : signal.smooth_spec(), 
                musical_matrix  : signal.musical_matrix.clone(), 
                spec            : signal.spec_string() + "\n" + file_name.as_str()})
    } else {
        Flex::column()
    }
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(format!("Radial Spectrum"))
        .window_size((600., 600.));

   AppLauncher::with_window(main_window)
      .launch(UI::new())
      .expect("launch failed");
}

