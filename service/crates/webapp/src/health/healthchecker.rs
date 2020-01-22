use std::boxed::Box;
use std::collections::HashMap;
use universe_health::Healthcheck;

/// Struct representing the way to check the health of the entire system
pub struct Healthchecker {
    checks: HashMap<String, Box<dyn Healthcheck>>,
}

/// Builder to make constructing a Healthchecker wasier
#[derive(Default)]
pub struct HealthcheckerBuilder {
    checks: HashMap<String, Box<dyn Healthcheck>>,
}

impl HealthcheckerBuilder {
    /// Add a new check to the healthchecker
    ///
    /// # Arguments
    /// * `component` The name of the component to add
    /// * `check` The check to add for this component
    ///
    /// # Returns
    /// The builder again, so that it can be chained
    pub fn add(mut self, component: &str, check: Box<dyn Healthcheck>) -> Self {
        self.checks.insert(component.to_owned(), check);
        self
    }

    /// Actually build the healthchecker from the checks that we've built up so far
    ///
    /// # Returns
    /// The built healthchecker, with the checks that we've built so far
    pub fn build(self) -> Healthchecker {
        Healthchecker {
            checks: self.checks,
        }
    }
}

impl Healthchecker {
    /// Check the health of the system
    ///
    /// This executes every check that we've got, and returns a map of their results
    ///
    /// # Returns
    /// The results of the healthchecks
    pub fn check_health(&self) -> HashMap<String, Result<String, String>> {
        let mut result = HashMap::new();

        for (k, v) in self.checks.iter() {
            result.insert(k.clone(), v.check_health());
        }

        result
    }
}
