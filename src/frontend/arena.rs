#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr;
use std::mem;

use crate::utils::string;

pub struct Arena {
    ptr: *mut u8,
    capacity: usize,
    offset: usize,
}

const DEFAULT_CAPACITY: usize = 1024;
const ARENA_ALIGNMENT: usize = 16;

pub unsafe fn arena_new(capacity: usize) -> Arena {
    let cap = if capacity == 0 { DEFAULT_CAPACITY } else { capacity };
    let layout = Layout::from_size_align(cap, ARENA_ALIGNMENT).unwrap();

    let ptr = alloc(layout);
    // assert!(!ptr.is_null(), "arena_new: allocation failed");
    // assert_eq!(
    //     ptr as usize % ARENA_ALIGNMENT,
    //     0,
    //     "arena.ptr is misaligned! Expected alignment: {}",
    //     ARENA_ALIGNMENT
    // );

    Arena {
        ptr,
        capacity: cap,
        offset: 0,
    }
}

pub unsafe fn arena_alloc(arena: *mut Arena, size: usize) -> *mut u8 {
    arena_alloc_align(arena, size, ARENA_ALIGNMENT)
}

pub unsafe fn arena_alloc_ty<T>(arena: *mut Arena) -> *mut T {
    let align = mem::align_of::<T>();
    let size = mem::size_of::<T>();
    let ptr = arena_alloc_align(arena, size, align);
    // assert_eq!(
    //     ptr as usize % align,
    //     0,
    //     "arena_alloc_ty: returned pointer is misaligned for type!"
    // );
    ptr as *mut T
}

pub unsafe fn arena_alloc_array<T>(arena: *mut Arena, count: usize) -> *mut T {
    let align = mem::align_of::<T>();
    let size = mem::size_of::<T>() * count;
    let ptr = arena_alloc_align(arena, size, align);
    // assert_eq!(
    //     ptr as usize % align,
    //     0,
    //     "arena_alloc_array: returned pointer is misaligned for type!"
    // );
    ptr as *mut T
}

pub unsafe fn arena_alloc_align(allocator: *mut Arena, size: usize, align: usize) -> *mut u8 {
    assert!(align.is_power_of_two(), "arena_alloc_align: align must be a power of two");

    let arena = &mut *allocator;
    let aligned_offset = (arena.offset + align - 1) & !(align - 1);
    let end = aligned_offset + size;

    if end > arena.capacity {
        // eprintln!(
        //     "arena_alloc_align: growing arena from {} to {} bytes",
        //     arena.capacity,
        //     arena.capacity * 2
        // );
        arena_grow(allocator, arena.capacity * 2);
        return arena_alloc_align(allocator, size, align); // try again after growth
    }

    let result = arena.ptr.add(aligned_offset);
    // debug_assert_eq!(
    //     result as usize % align,
    //     0,
    //     "arena_alloc_align: returned pointer is not properly aligned!"
    // );

    // log allocation info (can be removed in production)
    // eprintln!(
    //     "[arena_alloc_align] base: {:p}, aligned_offset: {}, align: {}, final ptr: {:p}",
    //     arena.ptr, aligned_offset, align, result
    // );

    arena.offset = end;
    result
}

pub unsafe fn arena_grow(allocator: *mut Arena, new_cap: usize) {
    let arena = &mut *allocator;
    let layout = Layout::from_size_align(arena.capacity, ARENA_ALIGNMENT).unwrap();
    let new_ptr = realloc(arena.ptr, layout, new_cap);

    // assert!(!new_ptr.is_null(), "arena_grow: realloc failed");
    // assert_eq!(
    //     new_ptr as usize % ARENA_ALIGNMENT,
    //     0,
    //     "arena_grow: reallocated pointer is misaligned!"
    // );

    arena.ptr = new_ptr;
    arena.capacity = new_cap;
}

pub unsafe fn arena_reset(allocator: *mut Arena) {
    (*allocator).offset = 0;
}

pub unsafe fn arena_free(allocator: *mut Arena) {
    let layout = Layout::from_size_align((*allocator).capacity, ARENA_ALIGNMENT).unwrap();
    dealloc((*allocator).ptr, layout);

    (*allocator).ptr = ptr::null_mut();
    (*allocator).capacity = 0;
    (*allocator).offset = 0;
}

pub unsafe fn arena_alloc_str(allocator: *mut Arena, s: &str) -> string::CogString {
    let bytes = s.as_bytes();
    let len = bytes.len();

    let dest = arena_alloc_align(allocator, len, ARENA_ALIGNMENT);
    ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);

    string::CogString {
        data: dest,
        len,
    }
}
