
use crate::frontend::ir::*;

pub struct NasmContext<'a> {
    insts: &'a LirModule<'a>,
    // Buffer to hold external declarations
    externals: String,
    // Buffer to hold function definitions
    functions: String,
    // Buffer to hold data section
    data_section: String,
}

impl<'a> NasmContext<'a> {
    pub fn new (irmod: &'a LirModule<'a>) -> Self {
	Self {
	    insts: irmod,
	    externals: String::new(),
	    functions: String::new(),
	    data_section:  String::new(),
	}
    }

    pub fn build_output (&mut self) -> String {
	let mut buffer = String::new();
	if self.externals.len() != 0 {
	    buffer += &self.externals;
	    buffer += "\n";
	}
	if self.functions.len() != 0 {
	    buffer += "section .text\n";
	    buffer += &self.functions;
	}
	if self.data_section.len() != 0 {
	    buffer += &self.data_section;

	}
	buffer += &format!(
	    "\nsection .note.GNU-stack\n"
	);
	buffer
    }
    
    pub fn generate (irmod: &'a LirModule<'a>) -> Option<Self> {
	let mut nctx = Self::new (irmod);

	for n in 0..nctx.insts.labels.len() {
	    let label = &nctx.insts.labels[n];
	    nctx.emit_label (label);
	}

	Some (nctx)
    }

    fn emit_label (&mut self, label: &LirLabel) {
	match label.kind {
	    LirLabelKind::Function => {
		self.emit_function (label);
	    }
	    LirLabelKind::GlobalLabel => {
		self.emit_global (label);
	    }
	    _ => todo!()
	}
    }

    fn emit_global (&mut self, label: &LirLabel) {
	if let LirLabelValue:: GlobalLabel (name) = label.value {
	    self.externals += &format!("\nglobal {}", name);
	}
    }
    
    fn emit_function (&mut self, label: &LirLabel) {
	match label.value {
	    LirLabelValue::Function (ref _lfn) => {
		self.functions += &format!("\n{}:", label.name);
		self.functions += &format!("\n\tpush rbp");
		self.functions += &format!("\n\tmov rbp, rsp");

		if label.name == "main" {
		    self.functions += &format!("\n\tmov eax, 0");
		}
		
		self.functions += &format!("\n\tpop rbp");
		self.functions += &format!("\n\tret");
	    }
	    _ => unreachable!()
	}
    }
}
