#![allow(unused)]

use crate::frontend::node:: {Node, Stmt, Expr, Item};
use crate::frontend::span:: {Spanned, Span};
use crate::frontend::symbols:: {FunctionInfo, FunctionTable};
use crate::frontend::context:: {Context};

pub struct Semantics<'a> {
    root:    &'a Node<'a>,
    source:  &'a str,

    root_context:    Context<'a>,
    current_context: &'a mut Context<'a>,
}

impl<'a> Semantics<'a> {
    pub fn new (source: &'a str, root: &'a Node<'a>) -> Self {
	let ctx = Context::new(None);
	Self {
	    root,
	    source,
	    current_context: &mut ctx,
	    root_context: ctx,
	}
    }

    pub fn enter_new_context(&mut self) {
	let ctx = Context::new(Some(self.current_context));
    }
    
    pub fn register_item(&mut self, item: &'a Spanned<'a, Box<Item<'a>>>) {
	
    }
    
    pub fn register_all_items(&mut self) {
	if let Node::Program (items) = self.root {
	    for item in items {
		self.register_item(item);
	    }
	}	
    }
    
    pub fn check (&mut self) -> bool {
	self.register_all_items();
	
	if let Node::Program (items) = self.root {
	    for item in items {
		self.check_item(item);
	    }
	} 
	
	return true;
    }
    
    pub fn check_item (&mut self, item: &'a Spanned<'a, Box<Item<'a>>>) {
	if let Item::Function (ref func) = *item.item {
	    println!("Function: {name}", name = func.name);
	}
    }
}
