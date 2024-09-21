use crate::ast::{Argument, Block, FrameDefinition, Identifier, SlotDefinition};
use crate::error::{arguments_span, Error, Result};
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
    Block(Block, Frame),
}

#[derive(Debug, Clone)]
struct Slot {
    index: usize,
    sub_frame: Option<Frame>,
}

pub enum Lookup {
    Slot(usize),
    Block(Block, Frame),
}

impl Frame {
    pub fn from_definition(definition: &FrameDefinition, scope: &Scope) -> Self {
        let mut symbols = HashMap::new();
        let mut index = 0;
        for slot in &definition.slots {
            match slot {
                SlotDefinition::Variable(name) => {
                    symbols.insert(
                        name.value.clone(),
                        Symbol::Slot(Slot {
                            index,
                            sub_frame: None,
                        }),
                    );

                    index += 1;
                }

                SlotDefinition::SubFrame(name, frame) => {
                    let sub_frame_definition =
                        scope.frame_definition(&frame.value).unwrap_or_else(|| {
                            panic!("Error: No frame '{}' found", frame.value);
                        });

                    // TODO: Detect cycles.
                    let sub_frame = Frame::from_definition(sub_frame_definition, scope);
                    let sub_frame_size = sub_frame.size();
                    symbols.insert(
                        name.value.clone(),
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
            name: definition.name.value.clone(),
            symbols,
        }
    }

    pub fn macro_frame(
        &self,
        name: &Identifier,
        parameters: &[Identifier],
        arguments: &[Argument],
    ) -> Result<Self> {
        if parameters.len() != arguments.len() {
            return Err(Error {
                span: arguments_span(arguments).unwrap_or(name.span),
                message: format!(
                    "Macro expected {} arguments, got {}",
                    parameters.len(),
                    arguments.len()
                ),
            });
        }

        let mut symbols = HashMap::new();
        for (name, argument) in parameters.iter().zip(arguments) {
            match argument {
                Argument::Variable(variable) => {
                    let (slot, index) = self.slot(variable)?;
                    symbols.insert(
                        name.value.clone(),
                        Symbol::Slot(Slot {
                            index,
                            sub_frame: slot.sub_frame.clone(),
                        }),
                    );
                }

                Argument::Block(block) => {
                    symbols.insert(
                        name.value.clone(),
                        Symbol::Block(block.clone(), self.clone()),
                    );
                }
            }
        }

        Ok(Self {
            name: self.name.clone(),
            symbols,
        })
    }

    fn slot(&self, path: &[Identifier]) -> Result<(&Slot, usize)> {
        if path.len() == 0 {
            panic!("Path must have at least one element");
        }

        let name = &path[0];
        let symbol = self.symbols.get(&name.value).ok_or(Error {
            span: name.span,
            message: format!(
                "No symbol with the name '{}' found in frame '{}'",
                name.value, self.name
            ),
        })?;

        if let Symbol::Slot(slot) = symbol {
            if path.len() > 1 {
                let sub_frame = slot.sub_frame.as_ref().ok_or(Error {
                    span: name.span,
                    message: "Must be a sub frame to use `.`".to_owned(),
                })?;

                let (sub_slot, sub_index) = sub_frame.slot(&path[1..])?;
                Ok((sub_slot, slot.index + sub_index))
            } else {
                Ok((slot, slot.index))
            }
        } else {
            Err(Error {
                span: name.span,
                message: format!("Expected symbol '{}' to be a slot", name.value),
            })
        }
    }

    pub fn lookup(&self, path: &[Identifier]) -> Result<Lookup> {
        if path.len() == 0 {
            panic!("Path must have at least one element");
        }

        let name = &path[0];
        let symbol = self.symbols.get(&name.value).ok_or(Error {
            span: name.span,
            message: format!(
                "No symbol with the name '{}' found in frame '{}'",
                name.value, self.name
            ),
        })?;

        Ok(match symbol {
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
