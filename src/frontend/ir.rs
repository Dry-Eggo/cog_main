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
	self.items.push (HirItem::Function (HirFunction { name, is_external: false}));
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
    pub is_external: bool,
}

impl<'a> HirFunction<'a> {
    pub fn set_external(&mut self) {
	self.is_external = true;
    }
}

//==================================LIR========================================\\
pub struct LirModule<'a> {
    pub labels: Vec<LirLabel<'a>>,
}

impl<'a> LirModule<'a> {
    pub fn new () -> Self {
	Self {
	    labels: vec![],
	}
    }

    pub fn get_label (&mut self, name: &'a str, kind: LirLabelKind) -> &mut LirLabel<'a> {
	match kind {
	    LirLabelKind::Function => {
		self.labels.push(LirLabel {
		    kind,
		    name,
		    value: LirLabelValue::Function (LirFunction::new()),
		    insts: None,
		});
	    }
	    LirLabelKind::GlobalLabel => {
		self.labels.push(LirLabel {
		    kind,
		    name,
		    value: LirLabelValue::GlobalLabel (name),
		    insts: None,
		});		
	    }
	    _ => todo!()
	}
	match self.labels.last_mut () {
	    Some (label) => label,
	    _ => unreachable!()
	}
    }
}

pub struct LirLabel<'a> {
    pub kind:  LirLabelKind,
    pub value: LirLabelValue<'a>,
    pub name: &'a str,

    pub insts: Option<Vec<LirInst>>,
}

impl<'a> LirLabel<'a> {
}

pub enum LirLabelKind {
    GlobalLabel,
    Function,
    Invalid,
}

pub enum LirLabelValue<'a> {
    Function (LirFunction),
    GlobalLabel (&'a str),
    Invalid,
}

pub struct LirFunction {
    pub is_external: bool,
}

impl LirFunction {
    pub fn new () -> Self {
	Self {
	    is_external: false,
	}
    }

    pub fn set_external(&mut self) {
	self.is_external = true;
    }
}

pub enum LirInst {
    Alloca {
	
    },
}

pub enum LirType {
    Qword,
    Dword,
    Word,
    Byte,
    Array (Vec<LirType>),
}
