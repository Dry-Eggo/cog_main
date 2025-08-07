

pub enum Item {

    FunctionDefinition ( FnDef ),
    
    /// Place Holder for Parsing Errors
    /// Won't ever reach semantics because syntax error will halt compilation
    /// before semantics
    Invalid,
}


/// Function Definition
pub struct FnDef {
    // points to its name in the source.
    // source lives longer than the compilation
    // so i think it's ok to use 'static
    name: &'static str,
}
