use autograd as ag;

pub struct ParseResult {
}
impl ParseResult {
   pub fn probability(&self) -> f64 {
      0.0
   }
}

pub struct ProbabilisticGrammar {
   dropdown_penalty: f64,
}
impl Default for ProbabilisticGrammar {
    fn default() -> Self {
       ProbabilisticGrammar {
          dropdown_penalty: 0.9
       }
    }
}
impl ProbabilisticGrammar {
   pub fn train<P: AsRef<std::path::Path>>(&self, p: P) {} 
   pub fn recognize(&self, cs: &str) -> ParseResult {
      ParseResult {}
   }
}

pub fn load_grammar(encf: &str) -> ProbabilisticGrammar {
   Default::default()
}
