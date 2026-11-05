#[derive(Debug, Clone)]
pub struct TestEntry {
    pub name: String,
    pub format: String,
}

impl TestEntry {
    pub fn new(name: impl Into<String>, format: impl Into<String>) -> Self {
        Self { name: name.into(), format: format.into() }
    }
}

#[derive(Debug, Default)]
pub struct OfficeTestCorpus {
    entries: Vec<TestEntry>,
}

impl OfficeTestCorpus {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, entry: TestEntry) { self.entries.push(entry); }
    pub fn count(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corpus_add() {
        let mut c = OfficeTestCorpus::new();
        c.add(TestEntry::new("roundtrip_docx", "docx"));
        assert_eq!(c.count(), 1);
    }
}
