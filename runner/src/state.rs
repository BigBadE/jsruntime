use std::collections::HashMap;
use shared_memory::Shmem;
use crate::logger::Logger;

pub struct JSRunnerState {
    pub global_context: v8::Global<v8::Context>,
    pub shared_memory: Option<Shmem>,
    pub modules: HashMap<String, (usize, usize)>,
    pub output: Logger
}

impl JSRunnerState {
    pub fn get_offset(&self, module: &str) -> usize {
        return self.modules.get(module).unwrap().0;
    }
}