use crate::ast::{Definition, FrameDefinition, Macro, Program};
use std::collections::HashMap;

pub struct Scope {
    frame_definitions: HashMap<String, FrameDefinition>,
    macros: HashMap<String, Macro>,
}

impl Scope {
    pub fn new(program: &Program) -> Self {
        let mut scope = Scope {
            frame_definitions: HashMap::new(),
            macros: HashMap::new(),
        };

        for definition in program {
            match definition {
                Definition::Frame(frame) => {
                    scope
                        .frame_definitions
                        .insert(frame.name.clone(), frame.clone());
                }
                Definition::Macro(macro_) => {
                    scope.macros.insert(macro_.name.clone(), macro_.clone());
                }
                _ => {}
            }
        }

        return scope;
    }

    pub fn frame_definition(&self, name: &str) -> Option<&FrameDefinition> {
        self.frame_definitions.get(name)
    }

    pub fn macro_(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }
}
