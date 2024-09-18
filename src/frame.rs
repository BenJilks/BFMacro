use crate::ast::{FrameDefinition, SlotDefinition};
use crate::scope::Scope;
use std::collections::HashMap;
use std::process::exit;

#[derive(Clone)]
pub struct Frame {
    pub name: String,
    slots: HashMap<String, Slot>,
}

#[derive(Clone)]
struct Slot {
    index: usize,
    sub_frame: Option<Frame>,
}

impl Frame {
    pub fn from_definition(definition: &FrameDefinition, scope: &Scope) -> Self {
        let mut slots = HashMap::new();
        let mut index = 0;
        for slot in &definition.slots {
            match slot {
                SlotDefinition::Variable(name) => {
                    slots.insert(
                        name.clone(),
                        Slot {
                            index,
                            sub_frame: None,
                        },
                    );

                    index += 1;
                }

                SlotDefinition::SubFrame(name, frame) => {
                    let sub_frame_definition =
                        scope.frame_definition(&frame).unwrap_or_else(|| {
                            println!("Error: No frame '{frame}' found");
                            exit(1);
                        });

                    // TODO: Detect cycles.
                    let sub_frame = Frame::from_definition(sub_frame_definition, scope);
                    let sub_frame_size = sub_frame.size();
                    slots.insert(
                        name.clone(),
                        Slot {
                            index,
                            sub_frame: Some(sub_frame),
                        },
                    );

                    index += sub_frame_size;
                }
            }
        }

        Self {
            name: definition.name.clone(),
            slots,
        }
    }

    pub fn macro_frame(&self, parameters: &[String], arguments: &[String]) -> Self {
        let mut slots = HashMap::new();
        for (name, variable) in parameters.iter().zip(arguments) {
            let slot = self.slots.get(variable).unwrap_or_else(|| {
                println!("Error: No variable '{variable}' in frame '{}'", self.name);
                exit(1);
            });

            slots.insert(name.clone(), slot.clone());
        }

        Self {
            name: self.name.clone(),
            slots,
        }
    }

    pub fn offset(&self, name: &str) -> Option<usize> {
        self.slots.get(name).map(|slot| slot.index)
    }

    pub fn size(&self) -> usize {
        self.slots
            .iter()
            .map(|(_, slot)| {
                if let Some(frame) = &slot.sub_frame {
                    slot.index + frame.size()
                } else {
                    slot.index + 1
                }
            })
            .max()
            .unwrap_or(0)
    }
}
