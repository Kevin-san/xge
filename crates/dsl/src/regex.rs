
use regex::Regex;
use anyhow::Result;

/// Regular expression pattern builder
pub struct RegexPattern {
    pattern: String,
}

impl RegexPattern {
    /// Creates a new regex pattern builder
    pub fn new() -> Self {
        RegexPattern {
            pattern: String::new(),
        }
    }
    
    /// Adds a literal string to the pattern
    pub fn literal(mut self, text: &str) -> Self {
        self.pattern.push_str(&regex::escape(text));
        self
    }
    
    /// Adds a character class to the pattern
    pub fn char_class(mut self, chars: &str) -> Self {
        self.pattern.push('[');
        self.pattern.push_str(chars);
        self.pattern.push(']');
        self
    }
    
    /// Adds a digit character class (\d)
    pub fn digit(mut self) -> Self {
        self.pattern.push_str("\\d");
        self
    }
    
    /// Adds a word character class (\w)
    pub fn word(mut self) -> Self {
        self.pattern.push_str("\\w");
        self
    }
    
    /// Adds a whitespace character class (\s)
    pub fn whitespace(mut self) -> Self {
        self.pattern.push_str("\\s");
        self
    }
    
    /// Adds a wildcard (.)
    pub fn any(mut self) -> Self {
        self.pattern.push('.');
        self
    }
    
    /// Adds a zero-or-more quantifier (*)
    pub fn zero_or_more(mut self) -> Self {
        self.pattern.push('*');
        self
    }
    
    /// Adds a one-or-more quantifier (+)
    pub fn one_or_more(mut self) -> Self {
        self.pattern.push('+');
        self
    }
    
    /// Adds a zero-or-one quantifier (?)
    pub fn zero_or_one(mut self) -> Self {
        self.pattern.push('?');
        self
    }
    
    /// Adds a specific quantifier {n}
    pub fn exactly(mut self, count: usize) -> Self {
        self.pattern.push_str(&format!("{{{}}}", count));
        self
    }
    
    /// Adds a range quantifier {n,m}
    pub fn range(mut self, min: usize, max: usize) -> Self {
        self.pattern.push_str(&format!("{{{},{}}}", min, max));
        self
    }
    
    /// Adds a start-of-string anchor (^)
    pub fn start(mut self) -> Self {
        self.pattern.push('^');
        self
    }
    
    /// Adds an end-of-string anchor ($)
    pub fn end(mut self) -> Self {
        self.pattern.push('$');
        self
    }
    
    /// Adds a group
    pub fn group<F>(mut self, f: F) -> Self where F: FnOnce(RegexPattern) -> RegexPattern {
        self.pattern.push('(');
        let group_pattern = f(RegexPattern::new());
        self.pattern.push_str(&group_pattern.pattern);
        self.pattern.push(')');
        self
    }
    
    /// Builds the regex pattern string
    pub fn build(&self) -> String {
        self.pattern.clone()
    }
    
    /// Compiles the pattern into a Regex
    pub fn compile(&self) -> Result<Regex> {
        Regex::new(&self.pattern).map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))
    }
    
    /// Compiles the pattern and tests it against the given text
    pub fn test(&self, text: &str) -> Result<bool> {
        let regex = self.compile()?;
        Ok(regex.is_match(text))
    }
}

