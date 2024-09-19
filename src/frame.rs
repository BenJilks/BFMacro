use crate::ast::{Argument, FrameDefinition, Instruction, SlotDefinition};
use crate::scope::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Frame {
    pub name: String,
    symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
enum Symbol {
    Slot(Slot),
    Block(Vec<Instruction>, Frame),
}

#[derive(Debug, Clone)]
struct Slot {
    index: usize,
    sub_frame: Option<Frame>,
}

pub enum Lookup {
    Slot(usize),
    Block(Vec<Instruction>, Frame),
}

impl Frame {
    pub fn from_definition(definition: &FrameDefinition, scope: &Scope) -> Self {
        let mut symbols = HashMap::new();
        let mut index = 0;
        for slot in &definition.slots {
            match slot {
                SlotDefinition::Variable(name) => {
                    symbols.insert(
                        name.clone(),
                        Symbol::Slot(Slot {
                            index,
                            sub_frame: None,
                        }),
                    );

                    index += 1;
                }

                SlotDefinition::SubFrame(name, frame) => {
                    let sub_frame_definition =
                        scope.frame_definition(&frame).unwrap_or_else(|| {
                            panic!("Error: No frame '{frame}' found");
                        });

                    // TODO: Detect cycles.
                    let sub_frame = Frame::from_definition(sub_frame_definition, scope);
                    let sub_frame_size = sub_frame.size();
                    symbols.insert(
                        name.clone(),
                        Symbol::Slot(Slot {
                            index,
                            sub_frame: Some(sub_frame),
                        }),
                    );

                    index += sub_frame_size;
                }
            }
        }

        Self {
            name: definition.name.clone(),
            symbols,
        }
    }

    pub fn macro_frame(&self, parameters: &[String], arguments: &[Argument]) -> Self {
        let mut symbols = HashMap::new();
        for (name, argument) in parameters.iter().zip(arguments) {
            match argument {
                Argument::Variable(variable) => {
                    let (slot, index) = self.slot(variable).unwrap_or_else(|| {
                        panic!("Error: No variable '{variable:?}' in frame '{}'", self.name);
                    });

                    symbols.insert(
                        name.clone(),
                        Symbol::Slot(Slot {
                            index,
                            sub_frame: slot.sub_frame.clone(),
                        }),
                    );
                }

                Argument::Block(block) => {
                    symbols.insert(name.clone(), Symbol::Block(block.clone(), self.clone()));
                }
            }
        }

        Self {
            name: self.name.clone(),
            symbols,
        }
    }

    fn slot(&self, path: &[String]) -> Option<(&Slot, usize)> {
        let name = &path[0];
        let symbol = self.symbols.get(name)?;
        if let Symbol::Slot(slot) = symbol {
            if path.len() > 1 {
                let sub_frame = slot
                    .sub_frame
                    .as_ref()
                    .expect("Must be a sub frame to use `.`");
                let (sub_slot, sub_index) = sub_frame.slot(&path[1..])?;
                Some((sub_slot, slot.index + sub_index))
            } else {
                Some((slot, slot.index))
            }
        } else {
            panic!("Expected {path:?} to be a slot");
        }
    }

    pub fn lookup(&self, path: &[String]) -> Option<Lookup> {
        if path.len() == 0 {
            return None;
        }

        let symbol = self.symbols.get(&path[0])?;
        Some(match symbol {
            Symbol::Block(block, frame) => Lookup::Block(block.clone(), frame.clone()),
            Symbol::Slot(_) => {
                let (_, index) = self.slot(path)?;
                Lookup::Slot(index)
            }
        })
    }

    pub fn size(&self) -> usize {
        self.symbols
            .iter()
            .map(|(_, symbol)| {
                if let Symbol::Slot(slot) = symbol {
                    if let Some(frame) = &slot.sub_frame {
                        slot.index + frame.size()
                    } else {
                        slot.index + 1
                    }
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0)
    }
}
