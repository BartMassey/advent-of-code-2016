use regex::*;

pub trait CapturesAtExt<'a> {
    fn at(&'a self, n: usize) -> Option<&'a str>;
}

impl<'a> CapturesAtExt<'a> for Captures<'a> {
    fn at(&'a self, n: usize) -> Option<&'a str> {
        self.get(n).map(|m| m.as_str())
    }
}
