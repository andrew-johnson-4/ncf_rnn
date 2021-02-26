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
   //0 means continue current open node
   //1 means open ROOT
   //n means open rule N
   //-n means close rule N
   //A ParseLine should contain exactly as many zeros as input characters
   //The ParseLine should also match each open rule to a closing rule at corresponding depth
   grammar: ProbabilisticGrammar,
   satisfied_rules: Vec<i64>,
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

#[derive(Clone)]
pub enum GrammarRule {
   //The first value in each enum tuple is the rule identifier
   //Identifiers are used to index the flattened ruleset NFA and to build ParseLines
   Node(i64,String,Rc<GrammarNode>),
   Seq(i64,String,Vec<Rc<GrammarRule>>),
   Any(i64,String,Vec<Rc<GrammarRule>>),
}


#[derive(Clone)]
pub struct ProbabilisticGrammar {
   //The dropdown_penalty hyper-parameter discourages parse lines that introduce many grammar nodes
   dropdown_penalty: f64,

   //The max_lines hyper-parameter is the maximum number of active parse lines for a parse attempt
   //Extra lines over this limit will be pruned based on their perplexity score
   max_lines: usize,

   grammar_rules: Option<GrammarRule>,

   //This is used for calculating posteriors, which are flattened into a DFA graph
   //This is not for parsing directly, because parsing is still Context Free
   grammar_tensor: Rc<Vec<Vec<f64>>>,

   //This index is for quick retrieval of grammar rules during parsing
   grammar_index: Rc<Vec<GrammarRule>>,
}

impl Default for ProbabilisticGrammar {
    fn default() -> Self {
       ProbabilisticGrammar {
          dropdown_penalty: 0.9,
          max_lines: 10_000,
          grammar_rules: None,
          grammar_tensor: Vec::new(),
          grammar_index: Vec::new(),
       }
    }
}

impl ProbabilisticGrammar {
   pub fn load<P: AsRef<std::path::Path>>(encf: P) -> Self {
      Default::default()
   }

   pub fn train<P: AsRef<std::path::Path>>(&self, dat: P) {} 

   pub fn recognize(&self, cs: &str) -> ParseResult {
      let mut lines: Vec<ParseLine> = if let Some(gr) = &self.grammar_rules {
         vec![ ParseLine {
            grammar: self.clone(),
            satisfied_rules: vec![1],
         } ]
      } else { vec![] };
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

