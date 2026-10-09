use std::ops::Range;

pub use regex::Regex;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Rule {
    pub regex: Regex,
    priority: u8,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub range: Range<usize>,
    pub content: String,
}

#[derive(Debug, Clone)]
struct Token {
    pub results: Vec<Range<usize>>,
}

pub struct Engine {
    rules: Vec<Rule>,
}

impl Engine {
    pub fn matches(&self, text: &str) {
        for rule in &self.rules {}
    }
}
