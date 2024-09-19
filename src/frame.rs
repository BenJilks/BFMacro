use crate::ast::{FrameDefinition, SlotDefinition, Variable};
use crate::scope::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Frame {
    pub name: String,
    slots: HashMap<String, Slot>,
}

#[derive(Debug, Clone)]
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
                            panic!("Error: No frame '{frame}' found");
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

    pub fn macro_frame(&self, parameters: &[String], arguments: &[Variable]) -> Self {
        let mut slots = HashMap::new();
        for (name, variable) in parameters.iter().zip(arguments) {
            let (slot, index) = self.get(variable).unwrap_or_else(|| {
                panic!("Error: No variable '{variable:?}' in frame '{}'", self.name);
            });

            slots.insert(
                name.clone(),
                Slot {
                    index,
                    sub_frame: slot.sub_frame.clone(),
                },
            );
        }

        Self {
            name: self.name.clone(),
            slots,
        }
    }

    fn get(&self, path: &[String]) -> Option<(&Slot, usize)> {
        let name = &path[0];
        let slot = self.slots.get(name)?;
        if path.len() > 1 {
            let sub_frame = slot
                .sub_frame
                .as_ref()
                .expect("Must be a sub frame to use `.`");
            let (sub_slot, sub_index) = sub_frame.get(&path[1..])?;
            Some((sub_slot, slot.index + sub_index))
        } else {
            Some((slot, slot.index))
        }
    }

    pub fn offset(&self, path: &[String]) -> Option<usize> {
        let (_, index) = self.get(path)?;
        Some(index)
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
