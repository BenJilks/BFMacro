use super::ast::{set_program_file_path, Definition, FrameDefinition, Macro, Program};
use super::macro_parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Scope {
    frame_definitions: HashMap<String, FrameDefinition>,
    macros: HashMap<String, Macro>,
    includes: HashSet<String>,
}

impl Scope {
    pub fn new(program: &Program, base_path: &Path) -> std::io::Result<Self> {
        let mut scope = Scope {
            frame_definitions: HashMap::new(),
            macros: HashMap::new(),
            includes: HashSet::new(),
        };

        scope.add_program(program, base_path)?;
        Ok(scope)
    }

    fn add_program(&mut self, program: &Program, base_path: &Path) -> std::io::Result<()> {
        for definition in program {
            match definition {
                Definition::Include(file_path) => {
                    if self.includes.contains(file_path) {
                        continue;
                    }
                    self.includes.insert(file_path.clone());

                    let file_path = base_path.join(file_path);
                    let mut file = File::open(&file_path)?;
                    let mut script = String::new();
                    file.read_to_string(&mut script)?;

                    let parser = macro_parser::ProgramParser::new();
                    let mut program = parser.parse(&script).unwrap();
                    set_program_file_path(&mut program, &file_path);

                    self.add_program(
                        &program,
                        &file_path
                            .parent()
                            .expect("You can't use the root directory as a bfm file"),
                    )?;
                }

                Definition::Frame(frame) => {
                    if self.frame_definitions.contains_key(&frame.name.value) {
                        panic!("Multiple definitions of {}", frame.name.value);
                    }

                    self.frame_definitions
                        .insert(frame.name.value.clone(), frame.clone());
                }

                Definition::Macro(macro_) => {
                    if self.macros.contains_key(&macro_.name.value) {
                        panic!("Multiple definitions of {}", macro_.name.value);
                    }

                    self.macros
                        .insert(macro_.name.value.clone(), macro_.clone());
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
