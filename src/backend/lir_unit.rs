
use crate::frontend::ir::*;

pub struct LirContext<'a> {
    hirmod: &'a HirModule<'a>,
    lirmod: LirModule<'a>,
}

impl<'a> LirContext<'a> {
    pub fn new (hmod: &'a HirModule<'a>) -> Self {
	Self {
	    hirmod: hmod,
	    lirmod: LirModule::new(),
	}
    }
    
    pub fn lower (hmod: &'a HirModule<'a>) -> Option<LirModule<'a>> {
	let mut lctx = Self::new(hmod);

	for n in 0..lctx.hirmod.items.len() {
	    let item = &lctx.hirmod.items[n];

	    lctx.lower_item (item);
	}
	
	Some (lctx.lirmod)
    }

    fn lower_item (&mut self, item: &HirItem<'a>) {
	match item {
	    HirItem:: Function (hfn) => {
		self.lower_function (hfn);
	    }
	    _ => {
	    }
	}
    }

    fn lower_function (&mut self, func: &HirFunction<'a>) {
	{
	    let mut _function_label = self.lirmod.get_label(func.name, LirLabelKind::Function);
	}
	if func.is_external {
	    self.lirmod.get_label(func.name, LirLabelKind::GlobalLabel);
	}
    }
}
