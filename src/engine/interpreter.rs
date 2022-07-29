use std::{fmt::Debug, path::Path};

use super::php;
use super::mixin::{Mixin, MixinTypes};

pub struct Interpreter {
    pub mixins: Vec<Mixin>
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            mixins: vec![]
        }
    }
}

impl Debug for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interpreter")
            .field("mixin count", &self.mixins.len())
            .finish()
    }
}

impl Interpreter {
    pub fn interpret(self: &mut Self, contents: &String, path: &Path) {
        let mut lines = contents.lines();
        let mut namespace: &str = "";
        let mut incomplete_mixin: Mixin = Mixin::new();
        let mut raw = "".to_string();
        let path = path.to_str().unwrap();
        while let Some(line) = lines.next() {
            // Start of processing
            if line.starts_with("namespace") {
                let mut name = php::extract_namespace(line);
                namespace = &mut name;
            }
            // I love if blocks in if blocks
            if incomplete_mixin.at != MixinTypes::None {
                // This if only executes if the interpreter is currently 
                // inside a mixin
                if line.starts_with("function") {
                    // Reset the raw counter
                    raw = "".to_string();
                    // Name the mixin
                    incomplete_mixin.name = php::extract_function_name(line).to_string();
                    incomplete_mixin.namespace = namespace.to_string();
                    incomplete_mixin.args = php::extract_function_params(line);
                    incomplete_mixin.path = path.to_string();
                } else {
                    if line.starts_with("}") {
                        incomplete_mixin.raw = raw.clone();
                        self.mixins.push(incomplete_mixin.clone());
                    } else {
                        raw = format!("{}{}", raw.clone(), line);
                    }
                }
            }
            if !line.starts_with("#@Inject") {
                continue;
            }
            // If there was a mixin without a namespace panic!
            if namespace == "" {
                panic!("Injection contained no namespace! Please include a PHP namespace on the second line...");
            }
            // Extract the mixin
            let mixin = Mixin::extract_type(line);
            incomplete_mixin = mixin;
        }
        self.mixins.sort_by(|a, b| {
            a.cmp(b)
        });
    }
}