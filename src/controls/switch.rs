use std::sync::{Arc, RwLock};

/// A switch
pub struct Switch {
    value: Arc<RwLock<bool>>,
}

/// Allow making copies of Knobs. There are Arcs underneath so when sharing a
/// copied Knob, a change to one will change them all.
impl Clone for Switch {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl Switch {
    pub fn new() -> Self {
        Self {
            value: Arc::new(RwLock::new(false)),
        }
    }

    pub fn set(&self, value: bool) {
        *self.value.write().unwrap() = value;
    }

    pub fn get(&self) -> bool {
        *self.value.read().unwrap()
    }
}