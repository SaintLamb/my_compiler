#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(s: impl Into<String>) -> Self {
        Symbol(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol(s.to_string())
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}