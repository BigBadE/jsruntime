use std::collections::HashMap;
use runner::imports::Provider;

pub fn global_provider() -> Provider {
    Provider {
        name: "core",
        functions: HashMap::new(),
        objects: HashMap::new()
    }
}