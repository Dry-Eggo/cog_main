#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::alloc:: { alloc, realloc, dealloc, Layout };
use std::ptr::   { self,  NonNull };
use std::mem;

use crate::utils::string;

pub struct Arena {
    ptr:     *mut u8,
    capacity: usize,
    offset:   usize,
}

const DEFAULT_CAPACITY: usize = 1024;

pub unsafe fn arena_new (capacity: usize) -> Arena {
    let cap = if capacity == 0 { DEFAULT_CAPACITY } else { capacity };
    let layout = Layout::from_size_align(cap, mem::align_of::<usize>()).unwrap();
    unsafe {
	let ptr    = alloc(layout);

	Arena {
	    ptr,
	capacity: cap,
	    offset: 0
	}
    }
}

pub unsafe fn arena_alloc(arena: *mut Arena, size: usize) -> *mut u8 {
    arena_alloc_align(arena, size, mem::align_of::<u8>())
}

pub unsafe fn arena_alloc_ty<T>(arena: *mut Arena) -> *mut T {
    arena_alloc_align(arena, mem::size_of::<T>(), mem::align_of::<T>()) as *mut T
}

pub unsafe fn arena_alloc_array<T>(arena: *mut Arena, count: usize) -> *mut T {
    arena_alloc_align(arena, mem::size_of::<T>() * count, mem::align_of::<T>()) as *mut T
}

pub unsafe fn arena_alloc_align (allocator: *mut Arena, size: usize, align: usize) -> *mut u8 {
    let arena = &mut *allocator;

    let aligned_offset = (arena.offset + align - 1) & !(align - 1);
    let end            = aligned_offset + size;

    if end > arena.capacity {
	arena_grow(allocator, arena.capacity*2);
	return arena_alloc_align (allocator, size, align);
    }

    let result = arena.ptr.add(aligned_offset);
    arena.offset = end;
    result
}

pub unsafe fn arena_grow(allocator: *mut Arena, new_cap: usize) {
    let arena = &mut *allocator;
    let layout = Layout::from_size_align(arena.capacity, mem::align_of::<usize>()).unwrap();
    let new_ptr = realloc(arena.ptr, layout, new_cap);

    arena.ptr = new_ptr;
    arena.capacity = new_cap;
}

pub unsafe fn arena_reset(allocator: *mut Arena) {
    (*allocator).offset = 0;
}

pub unsafe fn arena_free(allocator: *mut Arena) {
    let layout = Layout::from_size_align((*allocator).capacity, mem::align_of::<usize>()).unwrap();
    dealloc((*allocator).ptr, layout);
    (*allocator).ptr = ptr::null_mut();
    (*allocator).capacity = 0;
    (*allocator).offset   = 0;
}

pub unsafe fn arena_alloc_str(allocator: *mut Arena, s: &str) -> string::CogString {
    let bytes = s.as_bytes();
    let len   = bytes.len();

    let dest = arena_alloc_align(allocator, len, std::mem::align_of::<u8>());
    ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);

    string::CogString {
	data: dest,
	len,
    }
}
