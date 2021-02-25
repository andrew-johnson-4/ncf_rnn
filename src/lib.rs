use autograd as ag;

pub struct ParseResult {
}
impl ParseResult {
   pub fn probability(&self) -> f64 {
      0.0
   }
}

pub struct ProbabilisticGrammar {
}
impl ProbabilisticGrammar {
   pub fn recognize(&self, cs: &str) -> ParseResult {
      ParseResult {}
   }
}

pub fn load_grammar(encf: &str) -> ProbabilisticGrammar {
   ProbabilisticGrammar {}
}
