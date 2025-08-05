#![allow(unused)]

use std::alloc:: { alloc, Layout };
use std::ptr::   { self,  null_mut };

const DEFAULT_BLOCK_SIZE: usize = 1024*8;

pub struct Block {
    start:   *mut u8,
    current: *mut u8,
    end:     *mut u8,
}


impl Block {
    pub fn new(size: usize) -> Self {
	unsafe {
	    let layout = Layout::from_size_align(size, std::mem::align_of::<usize>()).unwrap();
	    let start  = alloc(layout);
	    if start.is_null() {
		panic!("Failed to allocate arena");
	    }

	    Self {
		start,
		current: start,
		end: start.add(size),
	    }
	}
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
	unsafe {
	    let aligned  = self.current.add(self.current.align_offset(align));
	    let new_curr = aligned.add(size);
	    if new_curr > self.end {
		return None;
	    }
	    self.current  = new_curr;
	    Some(aligned)
	}
    }
}

impl Drop for Block {
    fn drop(&mut self) {
	unsafe {
	    let size = self.end.offset_from(self.start) as usize;
	    let layout = Layout::from_size_align_unchecked(size, std::mem::align_of::<usize>());
	    std::alloc::dealloc(self.start, layout);
	}
    }
}

pub struct Allocator {
    blocks: Vec<Block>,
    block_size: usize,
}

impl Allocator {
    pub fn new(init_block_size: usize) -> Self {
	let mut arena = Self  {
	    blocks: vec![],
	    block_size: init_block_size,
	};
	arena.add_block();
	arena
    }

    pub fn add_block(&mut self) {
	self.blocks.push(Block::new(self.block_size));
    }

    pub fn alloc_align(&mut self, size: usize, align: usize) -> *mut u8 {
	if let Some(ptr) = self.blocks.last_mut().unwrap().alloc(size, align) {
	    return ptr;
	}

	let new_block_size = self.block_size.max(size.next_power_of_two());
	self.blocks.push(Block::new(new_block_size));
	self.blocks.last_mut().unwrap().alloc(size, align).unwrap()
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
	self.alloc_align(size, 8)
    }

    pub fn alloc_ty<T>(&mut self) -> *mut T {
	self.alloc_align(std::mem::size_of::<T>(), std::mem::align_of::<T>()) as *mut T
    }

    pub fn alloc_str(&mut self, s: &str) -> &str {
	let len = s.len();
	let ptr = self.alloc_align(len, std::mem::align_of::<u8>());

	unsafe {
	    std::ptr::copy_nonoverlapping(s.as_ptr(), ptr, len);
	    let slice = std::slice::from_raw_parts(ptr, len);
	    std::str::from_utf8_unchecked(slice)
	}
    }
}
