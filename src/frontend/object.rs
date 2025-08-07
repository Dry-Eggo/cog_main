
#[derive(Debug, Clone)]
pub struct FunctionInfo<'a> {
    name: &'a str,
}

#[derive(Debug, Clone)]
pub struct FunctionTable<'a> {
    entries: Vec<FunctionInfo<'a>>,
}

impl<'a> FunctionTable<'a> {
    pub fn new () -> Self {
	Self {
	    entries: vec![],
	}
    }

    pub fn make_function (&mut self, name: &'a str) -> usize {
	let id = self.entries.len();
	self.entries.push (FunctionInfo{
	    name
	});
	id
    }

    pub fn get_mut (&mut self, id: usize) -> Option<&mut FunctionInfo<'a>> {
	self.entries.get_mut (id)
    }

    pub fn get (&mut self, id: usize) -> Option<&FunctionInfo<'a>> {
	self.entries.get (id)
    }
}
