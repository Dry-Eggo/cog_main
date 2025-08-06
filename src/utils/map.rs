#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use std::ptr;

pub trait Hashable {
    fn hash(&self) -> usize;
}

const MAX_BUCKETS: usize = 35;

struct Bucket<K, V> {
    key:   *const K,
    value: *const V,
    next:  Option<*mut Bucket<K, V>>
}

/// A Non-Owning HashMap
pub struct CogMap<K, V> {
    entries: [Option<*mut Bucket<K, V>>; MAX_BUCKETS],
    arena:   *mut Arena
}

pub unsafe fn cogmap_new<K: Hashable, V> (arena: *mut Arena) -> *mut CogMap<K, V> {
    let map = arena_alloc_ty::<CogMap<K, V>>(arena);
    dref!(map).arena = arena;

    for i in 0..MAX_BUCKETS {
	dref!(map).entries[i] = None
    }
    
    map
}

/// Attempts to insert a value into the hashmap.
/// If value already exist, it returns a pointer to the original value and replaces it with the 'value'
/// else return None and inserts a new entry
pub unsafe fn cogmap_insert<K: Hashable + PartialEq, V> (map: *mut CogMap<K, V>, key: &K, value: &V) -> Option<*const V> {
    
    let index = key.hash() % MAX_BUCKETS;
    let mut head = &mut dref!(map).entries[index];

    let mut current = *head;
    while let Some(entry) = current {
	if *dref!(entry).key == *key {
	    let old_value = dref!(entry).value;
	    dref!(entry).value  = value;
	    return Some(old_value)
	}
	
	current = dref!(entry).next;
    }
    
    let new_bucket = arena_alloc_ty::<Bucket<K, V>>(dref!(map).arena);
    dref!(new_bucket).key   = key;
    dref!(new_bucket).value = value;
    dref!(new_bucket).next  = None;
    dref!(new_bucket).next  = *head;
    
    *head = Some(new_bucket);
	
    None
}

/// Attempts to fetch a value associated with the given 'key'.
/// Returns a const pointer to the value if it exists else returns None
pub unsafe fn cogmap_get<K: Hashable + PartialEq, V> (map: *mut CogMap<K, V>, key: &K) -> Option<*const V> {
    let index = key.hash() % MAX_BUCKETS;
    let mut head = &mut dref!(map).entries[index];

    let mut current = *head;
    while let Some(entry) = current {
	if *dref!(entry).key == *key {
	    let value = dref!(entry).value;
	    return Some(value)
	}
	
	current = dref!(entry).next;
    }
    None
}
