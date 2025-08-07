#![allow(unused)]
use std::collections::HashMap;

use crate::frontend::ast::*;
use crate::frontend::driver::*;
use crate::frontend::object::*;
use crate::frontend::token::*;
use crate::frontend::ir::*;

struct Context<'a> {
    ftable:    FunctionTable<'a>,
    functions: HashMap<String, usize>,
}

impl<'a> Context<'a> {
    pub fn new () -> Context<'a> {
	Self {
	    ftable: FunctionTable::new(),
	    functions: HashMap::new()
	}
    }

    pub fn add_function (&mut self, name: &'a str, span: Span) {
	let id = self.ftable.make_function (name, span);
	self.functions.insert(name.to_owned(), id);
    }
}

pub struct Semantics <'a> {
    driver: &'a Driver,
    root:    Vec<SpannedItem<'a>>,

    context_stack: Vec<Context<'a>>,
    pub irmod:         HirModule<'a>,
}

impl<'a> Semantics<'a> {
    fn new (driver: &'a Driver, ast: Vec<SpannedItem<'a>>) -> Self {
	let mut it = Self {
	    driver,
	    root: ast,
	    context_stack: vec![],
	    irmod: HirModule::new(),
	};
	it.context_stack.push (Context::new()); /* Parent Context */
	it
    }

    fn enter_context (&mut self) {
	self.context_stack.push(Context::new());
    }

    fn leave_context (&mut self) {
	self.context_stack.pop();
    }
    
    pub fn check (driver: &'a Driver, ast: Vec<SpannedItem<'a>>) -> Option<Semantics<'a>> {
	let mut sema = Semantics::new(driver, ast);
	
	sema.run_first_pass();
	sema.run_second_pass();

	Some (sema)
    }

    pub fn add_function (&mut self, name: &'a str, span: Span) {
	// All functions are stored on the Parent Context which is essentially the first Context
	let context = self.context_stack.first_mut ().unwrap();
	context.add_function (name, span);
    }
    
    fn run_first_pass (&mut self) -> Option<()> {
	for n in 0..self.root.len() {
	    let item = self.root[n];
	    
	    self.register_item (item);
	}
	None
    }

    fn run_second_pass (&mut self) -> Option<()> {
	for n in 0..self.root.len() {
	    let item = self.root[n];
	    
	    self.analyse_item (item);
	}
	None
    }

    fn register_item (&mut self, item: SpannedItem<'a>) {
	match item.item {
	    Item:: FunctionDefinition (fndef) => {
		self.register_function (fndef, item.span);
	    }
	    _ => {
		todo!()
	    }
	}	
    }
    
    fn register_function (&mut self, func: FnDef<'a>, span: Span) {
	self.add_function (func.name, span);
    }
    
    fn analyse_item (&mut self, item: SpannedItem<'a>) {
	match item.item {
	    Item:: FunctionDefinition (fndef) => {
		self.analyse_function (fndef, item.span);
	    }
	    _ => {
		todo!()
	    }
	}
    }

    fn analyse_function (&mut self, func: FnDef<'a>, _span: Span) {
	// Future api will allow for modification of this func_inst
	let mut func_inst = self.irmod.get_function(func.name).unwrap();
	func_inst.set_external();
    }
}
