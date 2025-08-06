#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr;
use std::mem;

use crate::utils::string;

struct AllocInfo {
    ptr: *mut u8,
    layout: Layout
}

impl AllocInfo {
    pub fn new (ptr: *mut u8, layout: Layout) -> Self {
	Self { ptr, layout }
    }
}

pub struct Arena {
    allocs: Vec<AllocInfo>
}

const DEFAULT_CAPACITY: usize = 1024;
const ARENA_ALIGNMENT: usize = 16;

pub unsafe fn arena_new(capacity: usize) -> Arena {
    // let layout = Layout::from_size_align(cap, ARENA_ALIGNMENT).unwrap();
    // 
    // let ptr = alloc(layout);
    // assert!(!ptr.is_null(), "arena_new: allocation failed");
    // assert_eq!(
    //     ptr as usize % ARENA_ALIGNMENT,
    //     0,
    //     "arena.ptr is misaligned! Expected alignment: {}",
    //     ARENA_ALIGNMENT
    // );

    Arena {
	allocs: vec![]
    }
}

pub unsafe fn arena_alloc(arena: *mut Arena, size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();
    let ptr    = alloc(layout);
    (*arena).allocs.push(AllocInfo::new(ptr, layout));
    ptr
}

pub unsafe fn arena_alloc_ty<T>(arena: *mut Arena) -> *mut T {
    let size = mem::size_of::<T>();
    let ptr = arena_alloc(arena, size);
    // assert_eq!(
    //     ptr as usize % align,
    //     0,
    //     "arena_alloc_ty: returned pointer is misaligned for type!"
    // );
    ptr as *mut T
}

pub unsafe fn arena_contruct<T> (arena: *mut Arena, x: T) -> *mut T {
    let ptr = arena_alloc_ty(arena);
    *ptr = x;
    ptr
}

pub unsafe fn arena_alloc_array<T>(arena: *mut Arena, count: usize) -> *mut T {
    let size = mem::size_of::<T>() * count;
    let ptr = arena_alloc(arena, size);
    // assert_eq!(
    //     ptr as usize % align,
    //     0,
    //     "arena_alloc_array: returned pointer is misaligned for type!"
    // );
    ptr as *mut T
}

pub unsafe fn arena_reset(arena: *mut Arena) {
    for allocation in &(*arena).allocs {
	dealloc(allocation.ptr, allocation.layout);
    }
    (*arena).allocs.clear();
}

pub unsafe fn arena_free(arena: *mut Arena) {
    arena_reset(arena)
}

pub unsafe fn arena_alloc_str(allocator: *mut Arena, s: &str) -> string::CogString {
    let bytes = s.as_bytes();
    let len = bytes.len();

    let dest = arena_alloc(allocator, len);
    ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);

    string::CogString {
        data: dest,
        len,
    }
}
