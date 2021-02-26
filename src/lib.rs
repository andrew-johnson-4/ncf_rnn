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
   pub fn load(encf: &str) -> Self {
      Default::default()
   }
   pub fn train<P: AsRef<std::path::Path>>(&self, p: P) {} 
   pub fn recognize(&self, cs: &str) -> ParseResult {
      //During parsing we merge Parse Lines that reach the same Grammar Vertex.
      //This keeps the size of our Parse Result from expanding over O(ns) with respect to
      //how many Grammar Vertexes we have defined (n) and the length of the parsed sentence (s).
      ParseResult {}
   }
}

