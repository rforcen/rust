// fast 2d matrix
//
// fast vector
//

#![allow(dead_code)]

use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Fast2D<T> {
    ptr: *mut T,
    len: usize,
    w: usize,
    h: usize,
}

unsafe impl<T> Send for Fast2D<T> {}

// unsafe index set/get

impl<T> Index<(usize, usize)> for Fast2D<T> {
    // index a=v[i]
    type Output = T;
    #[inline]
    fn index(&self, idx: (usize, usize)) -> &T {
        unsafe { &*(self.ptr.add(idx.0 + idx.1 * self.w)) }
    }
}

impl<T> IndexMut<(usize, usize)> for Fast2D<T> {
    // v[i]=a
    #[inline]
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut T {
        unsafe { &mut *(self.ptr.add(idx.0 + idx.1 * self.w)) }
    }
}

// iterator

impl<T: Copy> IntoIterator for Fast2D<T> {
    type Item = T;
    type IntoIter = Vec2dIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec2dIterator { v: self, index: 0 }
    }
}

pub struct Vec2dIterator<T> {
    v: Fast2D<T>,
    index: usize,
}

impl<T: Copy> Iterator for Vec2dIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.v.len {
            let ret = Some(unsafe { *(self.v.ptr.add(self.index)) });
            self.index += 1;
            ret
        } else {
            None
        }
    }
}

// mut iter

// Fast2D
impl<T> Fast2D<T> {
    pub fn new(w: usize, h: usize) -> Self {
        let len = w * h;
        let ptr = unsafe {
            alloc(Layout::from_size_align_unchecked(
                len * std::mem::size_of::<T>(),
                std::mem::size_of::<T>(),
            )) as *mut T
        };
        Self { ptr, w, h, len }
    }

    pub fn delete(&mut self) {
        if self.len > 0 {
            unsafe {
                dealloc(
                    self.ptr as *mut u8,
                    Layout::from_size_align_unchecked(
                        self.len * std::mem::size_of::<T>(),
                        std::mem::size_of::<T>(),
                    ),
                );
            }
            self.ptr = 0 as *mut T;
            self.len = 0;
            self.w = 0;
            self.h = 0;
        }
    }
    pub fn nbytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
    pub fn sizeof(&self) -> usize {
        std::mem::size_of::<T>()
    }

    // verified get/set
    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        let idx = x + y * self.w;
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get_ptr(&self, x: usize, y: usize) -> Option<*mut T> {
        let idx = x + y * self.w;
        if idx < self.len {
            unsafe { Some(self.ptr.add(idx)) }
        } else {
            None
        }
    }

    pub fn set(&self, x: usize, y: usize, d: T) {
        let idx = x + y * self.w;
        if idx < self.len {
            unsafe { *(self.ptr.add(idx)) = d }
        }
    }
    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.w
    }

    pub fn get_mut(&self, x: usize, y: usize) -> Option<&mut T> {
        let idx = x + y * self.w;
        if idx < self.len {
            unsafe { Some(&mut *(self.ptr.add(idx))) }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn n_cols(&self) -> usize {
        self.h
    }
    pub fn n_rows(&self) -> usize {
        self.w
    }
}

impl<T> Drop for Fast2D<T> {
    fn drop(&mut self) {
        self.delete()
    }
}

pub mod test_vec2d {
    use super::*;
    use std::time::Instant;

    #[test]
    fn create_destroy() {
        let mut fv = Fast2D::<f32>::new(1000, 1000);
        {
            let ff = Fast2D::<f64>::new(999, 999);
        }
        let mut s = 0_f32;
        for i in 0..fv.n_rows() {
            for j in 0..fv.n_cols() {
                fv[(i, j)] = i as f32;
            }
        }
        for f in fv {
            s += f;
        }
        println!("{}", s)
    }

    pub fn speed01() {
        let n_iter = 1000;
        let n_items = 1000;

        let t = Instant::now();
        let mut s: f64 = 0.;
        let mut vv = vec![vec![0_f64; n_items]; n_items];
        for _i in 0..n_iter {
            for j in 0..n_items {
                for k in 0..n_items {
                    vv[j][k] = 0.;
                    // s += 1.;
                }
            }
        }
        println!("Vec lap:{:?}->{s}", Instant::now() - t, s = s);

        let t = Instant::now();
        let mut vv = Fast2D::<f64>::new(n_items, n_items);
        let mut s: f64 = 0.;
        for _i in 0..n_iter {
            for j in 0..n_items {
                for k in 0..n_items {
                    vv[(j, k)] = 0.; //(j + n_items + k) as f64;
                    // s += 1.;
                }
            }
        }
        println!("Fast2d lap:{:?} -> {}", Instant::now() - t, s)
    }
    #[test]
    fn test_speed01_2d() {
        speed01()
    }
}
