pub type Program = Vec<Definition>;

#[derive(Debug, Clone)]
pub enum Definition {
    Frame(FrameDefinition),
    Macro(Macro),
    Using(Using),
}

#[derive(Debug, Clone)]
pub struct FrameDefinition {
    pub name: String,
    pub slots: Vec<SlotDefinition>,
}

#[derive(Debug, Clone)]
pub enum SlotDefinition {
    Variable(String),
    SubFrame(String, String),
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub parameters: Vec<String>,
    pub block: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Using {
    pub frame: String,
    pub block: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Subtract,
    Left,
    Right,
    Input,
    Output,
    OpenLoop,
    CloseLoop,
    Variable(String),
    MacroInvoke(String, Vec<String>),
}
