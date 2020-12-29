//
// fast vector
//

#![allow(dead_code)]

use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct FastVect<T> {
    ptr: *mut T,
    len: usize,
}

unsafe impl<T> Send for FastVect<T>{}

impl<T> Index<usize> for FastVect<T> { // index a=v[i]
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &T {
        unsafe { &*(self.ptr.add(idx)) }
    }
}

impl<T> IndexMut<usize> for FastVect<T> { // v[i]=a
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut T {
        unsafe { &mut *(self.ptr.add(idx)) }
    }
}

impl<T : Copy> IntoIterator for FastVect<T> { //  iterators
    type Item = T;
    type IntoIter = VectorIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator { v: self,  index: 0,  }
    }
}

pub struct VectorIntoIterator<T> {
	v : FastVect<T>,
	index : usize,
}

impl<T : Copy> Iterator for VectorIntoIterator<T> { // iterator
	
    type Item = T;

	#[inline]
    fn next(&mut self) -> Option<Self::Item>  {
		if self.index < self.v.len {
			let ret = Some(*&self.v[self.index]);
			self.index += 1;
			ret
		} else {		
			None
		}
    }
}

impl<T> FastVect<T> { // implementation

    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
           alloc(Layout::from_size_align_unchecked(len * std::mem::size_of::<T>(), std::mem::size_of::<T>())) as *mut T
		};
		Self { ptr, len }
    }

    pub fn delete(&mut self) {
        if self.len>0 {
			
            unsafe {
                dealloc(self.ptr as *mut u8, Layout::from_size_align_unchecked(self.len * std::mem::size_of::<T>(), std::mem::size_of::<T>()));     
            }            
            self.ptr=0 as *mut T;
            self.len=0;       
        }
    }
    
    pub fn nbytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
    
    pub fn sizeof(&self) -> usize {
        std::mem::size_of::<T>()
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

    pub fn set(&self, idx: usize, d:T) {
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
