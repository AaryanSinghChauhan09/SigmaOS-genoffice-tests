#[derive(Debug, Clone, PartialEq)]
pub enum TestResult { Pass, Fail(String), Skipped(String) }

#[derive(Debug, Clone)]
pub struct TestCase {
    pub id: String,
    pub category: String,
    pub description: String,
}

impl TestCase {
    pub fn new(id: impl Into<String>, category: impl Into<String>, description: impl Into<String>) -> Self {
        Self { id: id.into(), category: category.into(), description: description.into() }
    }
    pub fn run(&self) -> TestResult {
        TestResult::Pass
    }
}

#[derive(Debug, Default)]
pub struct TestCorpus { cases: Vec<TestCase> }
impl TestCorpus {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, case: TestCase) { self.cases.push(case); }
    pub fn count(&self) -> usize { self.cases.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_corpus_count() {
        let mut c = TestCorpus::new();
        c.add(TestCase::new("RT-01", "Format", "ODT round-trip"));
        assert_eq!(c.count(), 1);
    }
}
