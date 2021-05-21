// music freq
#![allow(dead_code)]

const MUSICAL_INC: f64 = 1.0594630943593; // 2^(1/12)
const LOG_MUSICAL_INC: f64 = 0.0577622650466;
const BASE_C0: f64 = 261.62556530061; // 440 * MUSICAL_INC^(-9)
const LOG_BASEC0: f64 = 5.5669143414923;
const LOG2: f64 = 0.6931471805599;

use crate::color_interp::*;
use std::f64::consts::E;

pub fn note_oct2freq(note: i32, oct: i32) -> f64 {
	BASE_C0 * (MUSICAL_INC).powf(note as f64 + 12. * oct as f64)
}

pub fn freq_in_octave(_freq: f64, oct: i32) -> f64 {
	let mut freq = _freq.abs();
	if freq != 0.0 {
		let fb = note_oct2freq(0_i32, oct);
		let ft = note_oct2freq(11_i32, oct); // rage freq in octave
		if freq > fb {
			while freq != 0. && freq > fb && !(freq >= fb && freq <= ft) {
				freq /= 2.
			}
		}
		// in octave
		else {
			while freq != 0. && freq < ft && !(freq >= fb && freq <= ft) {
				freq *= 2.
			}
		}
	}
	freq
}

pub fn freq2oct_note(_freq: f64) -> (i8, u8) {
	let lfb = if _freq != 0.0 {
		_freq.log(E) - LOG_BASEC0
	} else {
		0.0
	};
	let oct = (lfb / LOG2).floor();
	let note = (lfb / LOG_MUSICAL_INC - oct * 12.) as u8;

	(oct as i8, note)
}

pub fn freq2color(freq: f64) -> [f32; 3] {
	let (oct, _note) = freq2oct_note(freq); // get note and freq. err

	let f0 = note_oct2freq(0, oct as i32);
	let fz = note_oct2freq(0, oct as i32 + 1);
	let ratio = (freq - f0) / (fz - f0);
	default_interpolate(ratio as f32)
}
