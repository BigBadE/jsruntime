pub struct JSRunnerState {
    pub global_context: v8::Global<v8::Context>,
    pub output: i8
}