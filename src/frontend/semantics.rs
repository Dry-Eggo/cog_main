
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::frontend::ast::*;
use crate::frontend::driver::*;
use crate::frontend::object::*;
use crate::frontend::token::*;


type ContextPtr<'a> = Rc<RefCell<Context<'a>>>;

struct Context<'a> {
    ftable:    FunctionTable<'a>,
    functions: HashMap<String, usize>,
    parent:    Option<ContextPtr<'a>>
}

impl<'a> Context<'a> {
    pub fn new (parent: Option<ContextPtr<'a>>) -> ContextPtr<'a> {
	Rc::new ( RefCell::new (Self {
	    parent,
	    ftable: FunctionTable::new(),
	    functions: HashMap::new()
	}))
    }
}

pub struct Semantics <'a> {
    driver: &'a Driver,
    root:    Vec<SpannedItem<'a>>,

    context_stack: Vec<ContextPtr<'a>>,
}

impl<'a> Semantics<'a> {
    fn new (driver: &'a Driver, ast: Vec<SpannedItem<'a>>) -> Self {
	Self {
	    driver,
	    root: ast,
	    context_stack: vec![Context::new(None); 1] /* Root Context */
	}
    }

    fn enter_context (&mut self) {
	let current_ctx = self.context_stack.last ().unwrap();
	self.context_stack.push(Context::new(Some(current_ctx)));
    }
    
    pub fn check (driver: &Driver, ast: Vec<SpannedItem<'a>>) -> Option<()> {
	let mut sema = Semantics::new(driver, ast);
	
	sema.run_first_pass();
	sema.run_second_pass();
	
	None
    }

    fn run_first_pass (&mut self) -> Option<()> {
	// TODO: register function and generate C Header file
	None
    }

    fn run_second_pass (&mut self) -> Option<()> {
	for n in 0..self.root.len() {
	    let item = self.root[n];
	    
	    self.analyse_item (item);
	}
	None
    }

    fn analyse_item (&mut self, item: SpannedItem) {
	match item.item {
	    Item:: FunctionDefinition (fndef) => {
		self.analyse_function (fndef, item.span);
	    }
	    _ => {
		todo!()
	    }
	}
    }

    fn analyse_function (&mut self, func: FnDef, span: Span) {
	
    }
}
