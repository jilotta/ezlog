use chrono::{Datelike, Timelike, Utc};
use colored::Colorize;
pub mod map;
pub use map::Map;

/// The Logger object.
#[derive(Default, Clone)]
pub struct Logger {
    pub action: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
}

fn time_now() -> String {
    let now = Utc::now();
    format!(
        "{:02}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        now.year() % 100,
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond() / 1_000_000,
    )
}

impl Logger {
    /// Create a logger
    pub fn new() -> Self {
        Logger::default()
    }

    pub fn action<T: std::fmt::Display>(mut self, action: T) -> Self {
        self.action = Some(action.to_string());
        self
    }
    pub fn input<T: std::fmt::Display>(mut self, input: T) -> Self {
        self.input = Some(input.to_string());
        self
    }
    pub fn output<T: std::fmt::Display>(mut self, output: T) -> Self {
        self.output = Some(output.to_string());
        self
    }

    pub fn print(self, color: &str) -> Self {
        if let Some(action) = self.action.clone() {
            eprintln!(
                "{} [{}]",
                action.to_uppercase().color(color).bold(),
                time_now(),
            );
        }
        if let Some(input) = self.input.clone() {
            eprintln!(" >>> {}", input);
        }
        if let Some(output) = self.output.clone() {
            eprintln!(" <<< {}", output);
        }
        self
    }
    pub fn ok(self) -> Self {
        self.print("green")
    }
    pub fn warn(self) -> Self {
        self.print("yellow")
    }
    pub fn err(self) -> Self {
        self.print("red")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok() {
        Logger::new()
            .action("ok_test")
            .input(map![test1: "test1str"])
            .output(map![test2: 345345, test3: "sussy baka"])
            .ok();
    }
    #[test]
    fn le_grande_test() {
        use super::{map, Logger, Map};

        let default = Logger::new(); // empty
        let default2 = Logger::default(); // same as the above

        let default = default.ok(); // does nothing
        let default = default.action("test").ok(); // prints TEST: in green
        let default = default.ok();

        assert_eq!(
            map![test1: 234],
            Map(&[("test1".to_string(), "234".to_string())]) // auto conversion to String
        );

        let test2 = "This is a test.";
        assert_eq!(
            map![test2],
            Map(&[("test2".to_string(), "This is a test.".to_string())]) // auto expansion
        );

        // you can chain methods
        default
            .action("test2")
            .input(map![test2])
            .output(36) // not only maps
            .warn(); // prints the action in yellow

        default2
            .action("test")
            .action("third") // you can override the preferences set before
            .err();

        let really = true;
        Logger::new()
            .action("final")
            .input(map![what: "test"])
            .output(map![of_what: "of this lib", really])
            .print("purple");
    }
}
