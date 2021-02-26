use autograd as ag;

pub struct ParseResult {
}
impl ParseResult {
   pub fn probability(&self) -> f64 {
      0.0
   }
}

pub struct ProbabilisticGrammar {
   //The dropdown_penalty hyper-parameter discourages parse lines that introduce many grammar nodes
   dropdown_penalty: f64,

   //The max_lines hyper-parameter is the maximum number of active parse lines for a parse attempt
   //extra lines over this limit will be pruned based on their perplexity score
   max_lines: u64,
}
impl Default for ProbabilisticGrammar {
    fn default() -> Self {
       ProbabilisticGrammar {
          dropdown_penalty: 0.9,
          max_lines: 10_000,
       }
    }
}
impl ProbabilisticGrammar {
   pub fn load<P: AsRef<std::path::Path>>(encf: P) -> Self {
      Default::default()
   }
   pub fn train<P: AsRef<std::path::Path>>(&self, dat: P) {} 
   pub fn recognize(&self, cs: &str) -> ParseResult {
      ParseResult {}
   }
}

