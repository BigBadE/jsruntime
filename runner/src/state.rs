use shared_memory::Shmem;

pub(crate) struct JSRunnerState {
    pub(crate) global_context: v8::Global<v8::Context>,
    pub(crate) shared_memory: Option<Shmem>
}