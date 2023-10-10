use super::tweet::Tweet;

pub trait MyDisplay {
    fn summary(&self) -> String;
}

impl MyDisplay for Tweet {
    fn summary(&self) -> String {
        format!("default pretty summary...")
    }
}
