use tracing::debug;

pub struct Service {}

impl Service {
    pub fn new() -> Self {
        debug!("Building Universe...");
        Service {}
    }
}
