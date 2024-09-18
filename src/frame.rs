use crate::ast::FrameDefinition;
use std::{collections::HashMap, process::exit};

pub struct Frame {
    pub name: String,
    slots: HashMap<String, usize>,
}

impl Frame {
    pub fn macro_frame(&self, parameters: &[String], arguments: &[String]) -> Self {
        let mut slots = HashMap::new();
        for (name, variable) in parameters.iter().zip(arguments) {
            let offset = self.offset(&variable).unwrap_or_else(|| {
                println!("Error no variable '{variable}' in frame '{}'", self.name);
                exit(1);
            });

            slots.insert(name.clone(), offset);
        }

        Self {
            name: self.name.clone(),
            slots,
        }
    }

    pub fn offset(&self, name: &str) -> Option<usize> {
        self.slots.get(name).cloned()
    }
}

impl From<&FrameDefinition> for Frame {
    fn from(definition: &FrameDefinition) -> Self {
        let mut slots = HashMap::new();
        for (i, slot) in definition.slots.iter().enumerate() {
            slots.insert(slot.clone(), i);
        }

        Self {
            name: definition.name.clone(),
            slots,
        }
    }
}
