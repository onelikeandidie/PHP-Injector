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
        let path = path.to_str().unwrap();
        while let Some(line) = lines.next() {
            // Start of processing
            if line.starts_with("namespace") {
                let mut name = php::extract_namespace(line);
                namespace = &mut name;
            }
            if incomplete_mixin.at != MixinTypes::None {
                if line.starts_with("function") {
                    // Name the mixin
                    let complete_mixin = Mixin {
                        name: php::extract_function_name(line).to_string(),
                        namespace: namespace.to_string(),
                        at: incomplete_mixin.at.clone(),
                        args: php::extract_function_params(line),
                        target: incomplete_mixin.target.clone(),
                        path: path.to_string()
                    };
                    self.mixins.push(complete_mixin);
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