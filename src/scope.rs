use crate::ast::{Definition, FrameDefinition, Macro, Program};
use crate::macro_parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub struct Scope {
    frame_definitions: HashMap<String, FrameDefinition>,
    macros: HashMap<String, Macro>,
    includes: HashSet<String>,
}

impl Scope {
    pub fn new(program: &Program) -> std::io::Result<Self> {
        let mut scope = Scope {
            frame_definitions: HashMap::new(),
            macros: HashMap::new(),
            includes: HashSet::new(),
        };

        scope.add_program(program)?;
        Ok(scope)
    }

    fn add_program(&mut self, program: &Program) -> std::io::Result<()> {
        for definition in program {
            match definition {
                Definition::Include(file_path) => {
                    if self.includes.contains(file_path) {
                        continue;
                    }

                    let mut file = File::open(file_path)?;
                    let mut script = String::new();
                    file.read_to_string(&mut script)?;

                    let parser = macro_parser::ProgramParser::new();
                    let program = parser.parse(&script).unwrap();

                    self.includes.insert(file_path.clone());
                    self.add_program(&program)?;
                }

                Definition::Frame(frame) => {
                    if self.frame_definitions.contains_key(&frame.name) {
                        panic!("Multiple definitions of {}", frame.name);
                    }

                    self.frame_definitions
                        .insert(frame.name.clone(), frame.clone());
                }

                Definition::Macro(macro_) => {
                    if self.macros.contains_key(&macro_.name) {
                        panic!("Multiple definitions of {}", macro_.name);
                    }

                    self.macros.insert(macro_.name.clone(), macro_.clone());
                }

                _ => {}
            }
        }

        Ok(())
    }

    pub fn frame_definition(&self, name: &str) -> Option<&FrameDefinition> {
        self.frame_definitions.get(name)
    }

    pub fn macro_(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }
}
