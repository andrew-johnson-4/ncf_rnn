use autograd as ag;
use std::rc::Rc;

pub struct ParseResult {
   grammar: ProbabilisticGrammar,
   result: Vec<ParseLine>
}

impl ParseResult {
   pub fn probability(&self) -> f64 {
      self.result.get(0).map_or(0., |l| l.probability())
   }
}


pub struct ParseLine {
   grammar: ProbabilisticGrammar,
}

impl ParseLine {
   pub fn poke(&self, c: char) -> Vec<ParseLine> {
      Vec::new()
   }

   pub fn probability(&self) -> f64 {
      1.0
   }
}

pub struct GrammarNode {
   accepts: Box<Fn(&Vec<char>) -> bool>,
   terminal: bool,
}

pub enum GrammarRule {
   Seq(Vec<GrammarNode>),
   Any(Vec<GrammarRule>),
}

#[derive(Clone)]
pub struct ProbabilisticGrammar {
   //The dropdown_penalty hyper-parameter discourages parse lines that introduce many grammar nodes
   dropdown_penalty: f64,

   //The max_lines hyper-parameter is the maximum number of active parse lines for a parse attempt
   //Extra lines over this limit will be pruned based on their perplexity score
   max_lines: usize,

   grammar_rules: Option<Rc<GrammarRule>>,
}

impl Default for ProbabilisticGrammar {
    fn default() -> Self {
       ProbabilisticGrammar {
          dropdown_penalty: 0.9,
          max_lines: 10_000,
          grammar_rules: None
       }
    }
}

impl ProbabilisticGrammar {
   pub fn load<P: AsRef<std::path::Path>>(encf: P) -> Self {
      Default::default()
   }

   pub fn train<P: AsRef<std::path::Path>>(&self, dat: P) {} 

   pub fn recognize(&self, cs: &str) -> ParseResult {
      let mut lines: Vec<ParseLine> = vec![ /*grammar ROOT*/ ];
      for c in cs.chars() {
         let mut new_lines: Vec<ParseLine> = Vec::new();
         for l in lines.iter() {
            new_lines.append(&mut l.poke(c));
         }
         new_lines.sort_by(|a,b| a.probability().partial_cmp(&b.probability()).unwrap() );
         new_lines.reverse();
         new_lines.truncate(self.max_lines);
         lines = new_lines;
      }
      ParseResult {
         grammar: self.clone(),
         result: lines
      }
   }
}

