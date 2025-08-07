#![allow(unused)]
pub struct HirModule<'a> {
    pub items: Vec<HirItem<'a>>,    
}

impl<'a> HirModule<'a> {
    pub fn new () -> Self {
	Self {
	    items: vec![],
	}
    }

    pub fn get_function (&mut self, name: &'a str) -> Option<&mut HirFunction<'a>> {
	self.items.push (HirItem::Function (HirFunction { name }));
	if let Some (HirItem::Function (func)) = self.items.last_mut () {
	    return Some (func)
	}	
	None
    }
}

#[derive(Debug)]
pub enum HirItem<'a> {
    Function ( HirFunction<'a> ),
    Invalid,
}

#[derive(Debug)]
pub struct HirFunction<'a> {
    pub name: &'a str,
}
