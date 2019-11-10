use crate::error::{PakError, PakResult};
use crate::Model;
use std::any::Any;

pub trait Target: Any {
    fn name(&self) -> &'static str;
    fn generate_from(&self, model: Model) -> PakResult<()>;
}

#[derive(Default)]
pub struct TargetRepository {
    targets: Vec<Box<dyn Target>>,
}

unsafe impl Sync for TargetRepository {}

impl TargetRepository {
    pub fn new() -> TargetRepository { TargetRepository { targets: Vec::new() } }

    pub fn add(&mut self, target: Box<dyn Target>) -> PakResult<()> {
        self.targets.push(target);
        Ok(())
    }

    pub fn find(&self, target_name: &str) -> PakResult<&Box<dyn Target>> {
        if let Some(target) = self.targets.iter().find(|t| t.name() == target_name) {
            Ok(target)
        } else {
            Err(PakError::TargetNotFound(String::from(target_name)))
        }
    }
}
