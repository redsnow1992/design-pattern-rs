use super::{adaptee::SpecificTarget, target::Target};

pub struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        // Here's the "adaptation" of a specific output to a compatible output.
        self.adaptee.specific_request().chars().rev().collect()
    }
}