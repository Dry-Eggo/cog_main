#![allow(unused)]

use crate::frontend::node:: {Node, Stmt, Expr, Item, self};
use crate::frontend::span:: {Spanned, Span};
use crate::frontend::symbols:: {FunctionInfo, FunctionTable};
use crate::frontend::context:: {Context, SharedContext};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Semantics<'a> {
    root:    &'a Node<'a>,
    source:  &'a str,

    root_context:    SharedContext<'a>,
    current_context: SharedContext<'a>,
}

impl<'a> Semantics<'a> {
    pub fn new (source: &'a str, root: &'a Node<'a>) -> Self {
	let ctx = Rc::new(RefCell::new(Context::new(None)));
	Self {
	    root,
	    source,
	    current_context: Rc::clone(&ctx),
	    root_context: Rc::clone(&ctx),
	}
    }

    fn enter_new_context(&mut self) {
	let new_ctx = Rc::new(RefCell::new(Context::new(Some(Rc::clone(&self.current_context)))));
	self.current_context = new_ctx;
    }

    fn exit_context(&mut self)	{
	let parent = {
	    let current = self.current_context.borrow();
	    current.parent.as_ref().map(Rc::clone)
	};

	if let Some(p) = parent {
	    self.current_context = p;
	} else {
	    self.current_context = Rc::clone(&self.root_context);
	}
    }

    fn register_function(&mut self, func: &'a node::Function<'a>, span: &'a Span<'a>) {
	let finfo = FunctionInfo::new(&func.name, span.clone());
	self.current_context.borrow_mut().add_function(&func.name, finfo);
    }
    
    fn register_item(&mut self, item: &'a Spanned<'a, Box<Item<'a>>>) {
	if let Item::Function (ref func) = *item.item {
	    self.register_function(func, &item.span);
	}
    }
    
    fn register_all_items(&mut self) {
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
