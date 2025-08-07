
use crate::frontend::token::Spanned;

pub type SpannedItem<'a> = Spanned<Item<'a>>;

#[derive(Clone, Copy)]
pub enum Item<'a> {

    FunctionDefinition ( FnDef<'a> ),
    
    /// Place Holder for Parsing Errors
    /// Won't ever reach semantics because syntax error will halt compilation
    /// before semantics
    Invalid,
}


/// Function Definition
#[derive(Clone, Copy)]
pub struct FnDef<'a> {
    // points to its name in the source.
    // source lives longer than the compilation
    pub name: &'a str,
}
