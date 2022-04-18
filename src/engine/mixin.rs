use std::{fmt::Debug, collections::HashMap};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Mixin {
    pub name: String,
    pub namespace: String,
    pub at: MixinTypes,
    pub args: Vec<String>,
    pub target: String,
    pub path: String,
}

impl Debug for Mixin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mixin")
            .field("name",      &self.name)
            .field("namespace", &self.namespace)
            .field("at",        &self.at)
            .field("target",    &self.target)
            .finish()
    }
}

impl Ord for Mixin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.at.cmp(&other.at)
    }
}

impl Mixin {
    pub fn new() -> Self {
        Self { 
            name: Default::default(), 
            namespace: Default::default(), 
            at: MixinTypes::None, 
            args: vec![],
            target: Default::default(),
            path: Default::default()
        }
    }

    pub fn extract_type(line: &str) -> Self {
        let mut mixin = Self::new();
        let args = Self::extract_arguments(line);
        let at = args.get("at").unwrap().replace("\"", "");
        let target = args.get("target").unwrap().replace("\"", "");
        mixin.at = MixinTypes::get(&at, &args);
        mixin.target = target;
        return mixin;
    }

    fn extract_arguments(feed: &str) -> HashMap<String, String> {
        let mut chars = feed.chars();
        let mut is_in_arguments = false;
        let mut is_in_string = false;
        let mut char_buf = "".to_string();
        let mut result = HashMap::new();
        while let Some(character) = chars.next() {
            if character == '(' { is_in_arguments = true; }
            if !is_in_arguments { continue; }
            if character == '\"' {
                is_in_string = !is_in_string;
            }
            if is_in_string || (character != ' ' && character != '(' && character != ')' && character != ',') {
                char_buf.push(character);
            }
            if !is_in_string {
                if character == ',' || character == ')' {
                    // Split value pair
                    let split: Vec<&str> = char_buf.as_str().split("=").collect();
                    result.insert(split[0].to_string(), split[1].to_string());
                    // Remove the extracted pair
                    char_buf = "".to_string();
                }
                if character == ')' { is_in_arguments = false; }
            }
        };
        if is_in_string || is_in_arguments {
            panic!("Mixin not complete! \"{}\" is missing a ')' or a '\"'", feed);
        }
        return result;
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum MixinTypes {
    None,
    Head(MixinHead),
    Slice(MixinSlice),
    Prepend(MixinPrepend),
    Replace(MixinReplace),
    Append(MixinAppend),
    Tail(MixinTail)
}

impl Ord for MixinTypes {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (_, Self::None) => Ordering::Less,
            (Self::None, _) => Ordering::Greater,
            // Slices first
            (_, Self::Slice(_)) => Ordering::Greater,
            (Self::Slice(_), _) => Ordering::Less,
            // Replaces, Prepends and Appends next
            (Self::Prepend(_), Self::Replace(_)) => Ordering::Equal,
            (Self::Prepend(_), Self::Append(_)) => Ordering::Equal,
            (Self::Replace(_), Self::Prepend(_)) => Ordering::Equal,
            (Self::Replace(_), Self::Append(_)) => Ordering::Equal,
            (Self::Append(_), Self::Replace(_)) => Ordering::Equal,
            (Self::Append(_), Self::Prepend(_)) => Ordering::Equal,
            // Head and Tail last
            (_, Self::Head(_)) => Ordering::Less,
            (_, Self::Tail(_)) => Ordering::Less,
            (Self::Head(_), _) => Ordering::Greater,
            (Self::Tail(_), _) => Ordering::Greater,
            (_,_) => Ordering::Equal
        }
    }
}

impl MixinTypes {
    pub fn get(at: &str, args: &HashMap<String, String>) -> Self {
        match at {
            "HEAD"      => {
                let mut mixin = MixinHead::default();
                mixin.fill_params(args);
                return MixinTypes::Head(mixin);
            },
            "SLICE"     => {
                let mut mixin = MixinSlice::default();
                mixin.fill_params(args);
                return MixinTypes::Slice(mixin);
            },
            "PREPEND"   => {
                let mut mixin = MixinPrepend::default();
                mixin.fill_params(args);
                return MixinTypes::Prepend(mixin);
            },
            "REPLACE"   => {
                let mut mixin = MixinReplace::default();
                mixin.fill_params(args);
                return MixinTypes::Replace(mixin);
            },
            "APPEND"    => {
                let mut mixin = MixinAppend::default();
                mixin.fill_params(args);
                return MixinTypes::Append(mixin);
            },
            "TAIL"      => {
                let mut mixin = MixinTail::default();
                mixin.fill_params(args);
                return MixinTypes::Tail(mixin);
            },
            _ => MixinTypes::None
        }
    }
}

pub trait MixinType {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self;
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinNone;
impl MixinType for MixinNone {
    fn fill_params(self: &mut Self, _args: &HashMap<String, String>) -> &mut Self {self}
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinHead {
    pub offset: i32
}
impl MixinType for MixinHead {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.offset = args.get("offset")
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>().unwrap();
        return self;
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinSlice {
    pub from: i32,
    pub to: i32
}
impl MixinType for MixinSlice {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.from   = args.get("from")  .unwrap().parse().unwrap();
        self.to     = args.get("to")    .unwrap().parse().unwrap();
        return self;
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinPrepend {
    pub search: String,
    pub offset: i32
}
impl MixinType for MixinPrepend {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.search = args.get("search").unwrap().to_owned();
        self.offset = args.get("offset")
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>().unwrap();
        return self;
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinReplace {
    pub search: String,
    pub offset: i32
}
impl MixinType for MixinReplace {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.search = args.get("search").unwrap().to_owned();
        self.offset = args.get("offset")
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>().unwrap();
        return self;
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinAppend {
    pub search: String,
    pub offset: i32
}
impl MixinType for MixinAppend {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.search = args.get("search").unwrap().to_owned();
        self.offset = args.get("offset")
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>().unwrap();
        return self;
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd)]
pub struct MixinTail {
    pub offset: i32
}
impl MixinType for MixinTail {
    fn fill_params(self: &mut Self, args: &HashMap<String, String>) -> &mut Self {
        self.offset = args.get("offset")
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>().unwrap();
        return self;
    }
}