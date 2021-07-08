//
// fast vector
//

#![allow(dead_code)]

use num::traits::{Float, Zero};
use num_traits::cast::FromPrimitive;
use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::{Index, IndexMut};

fn calloc<T>(len: usize) -> *mut T {
    unsafe {
        alloc(Layout::from_size_align_unchecked(
            len * std::mem::size_of::<T>(),
            std::mem::size_of::<T>(),
        )) as *mut T
    }
}

fn free<T>(ptr: *mut T, len: usize) {
    unsafe {
        dealloc(
            ptr as *mut u8,
            Layout::from_size_align_unchecked(
                len * std::mem::size_of::<T>(),
                std::mem::size_of::<T>(),
            ),
        )
    }
}

#[derive(Clone, Debug)]
pub struct FastVect<T> {
    ptr: *mut T,
    len: usize,
}

// FastVect
impl<T: Float + FromPrimitive + Zero> FastVect<T> {
    pub fn new(len: usize) -> Self {
        Self {
            ptr: calloc(len),
            len,
        }
    }

    pub fn delete(&mut self) {
        if self.len > 0 {
            free(self.ptr, self.len);
            self.ptr = 0 as *mut T;
            self.len = 0;
        }
    }
    pub fn nbytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
    pub fn sizeof(&self) -> usize {
        std::mem::size_of::<T>()
    }

    // verified get/set
    pub fn at(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get_ptr(&self, idx: usize) -> Option<*mut T> {
        if idx < self.len {
            unsafe { Some(self.ptr.add(idx)) }
        } else {
            None
        }
    }

    pub fn set(&self, idx: usize, d: T) {
        if idx < self.len {
            unsafe { *(self.ptr.add(idx)) = d }
        }
    }

    pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            unsafe { Some(&mut *(self.ptr.add(idx))) }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

unsafe impl<T> Send for FastVect<T> {}

// unsafe index set/get

impl<T> Index<usize> for FastVect<T> {
    // index a=v[i]
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &T {
        unsafe { &*(self.ptr.add(idx)) }
    }
}

impl<T> IndexMut<usize> for FastVect<T> {
    // v[i]=a
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut T {
        unsafe { &mut *(self.ptr.add(idx)) }
    }
}

// iterator

impl<T: Copy> IntoIterator for FastVect<T> {
    type Item = T;
    type IntoIter = VectorIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator { v: self, index: 0 }
    }
}

pub struct VectorIntoIterator<T> {
    v: FastVect<T>,
    index: usize,
}

impl<T: Copy> Iterator for VectorIntoIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.v.len {
            let ret = Some(*&self.v[self.index]);
            self.index += 1;
            ret
        } else {
            None
        }
    }
}

// mut iter

// drop
impl<T> Drop for FastVect<T> {
    fn drop(&mut self) {
        free(self.ptr, self.len)
    }
}

// +, +=, -, -=, *, *=, /, /=   operator overload

impl<T: Add<Output = T> + Float + FromPrimitive> Add for FastVect<T>
where
    T: Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.len == other.len {
            let mut s = Self::new(self.len);
            for i in 0..self.len {
                s[i] = self[i] + other[i]
            }
            s
        } else {
            panic!("different sizes on +")
        }
    }
}

impl<T: Add<Output = T> + Float + FromPrimitive> Add<T> for FastVect<T>
where
    T: Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let mut s = FastVect::new(self.len);
        for i in 0..self.len {
            s[i] = self[i] + rhs
        }
        s
    }
}

impl<T: AddAssign + Float> AddAssign for FastVect<T>
where
    T: Copy,
{
    fn add_assign(&mut self, other: Self) {
        for i in 0..self.len {
            self[i] = self[i] + other[i]
        }
    }
}

impl<T: AddAssign> AddAssign<T> for FastVect<T>
where
    T: Copy,
{
    fn add_assign(&mut self, rhs: T) {
        for i in 0..self.len {
            self[i] += rhs
        }
    }
}
// test
pub mod test {
    use super::*;
    use std::time::Instant;

    #[test]
    fn create_destroy() {
        let mut fv = FastVect::<f32>::new(100000);
        {
            let ff = FastVect::<f64>::new(999999);
        }
        let mut s = 0_f32;
        for i in 0..fv.len() {
            fv[i] = i as f32;
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
        for i in 0..n_iter {
            let mut vv = vec![0_f64; n_items];
            for j in 0..vv.len() {
                vv[j] = 0.
            }
        }
        println!("Vec lap:{:?}", Instant::now() - t);

        let t = Instant::now();
        for _i in 0..n_iter {
            let mut vv = FastVect::<f64>::new(n_items);
            for j in 0..vv.len() {
                vv[j] = 0.
            }
        }
        println!("FastVec lap:{:?}", Instant::now() - t)
    }
    #[test]
    fn test_speed01() {
        speed01()
    }
    #[test]
    fn test_add() {
        let n_items = 1000;
        let mut vv = FastVect::<f64>::new(n_items);
        let mut vv1 = FastVect::<f64>::new(n_items);
        let mut vv2 = FastVect::<f64>::new(n_items);
        let l = vv.len();
        for i in 0..l {
            vv[i] = i as f64;
            vv1[i] = i as f64;
            vv2[i] = i as f64;
        }
        let mut vs = vv + vv1;
        for i in 0..l {
            assert_eq!(vs[i], (i + i) as f64)
        }

        vs += vv2;
    }
}
