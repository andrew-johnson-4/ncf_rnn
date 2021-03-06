use autograd::ndarray::Array2;
use std::rc::Rc;

pub struct ParseResult {
   pub grammar: Rc<ProbabilisticGrammar>,
   pub result: Vec<ParseLine>
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
   //Additionally each rule always gets its own number, even if the rhs is the same
   //For example, each rule in a sequence is given a unique numerical identifier
   pub grammar: Rc<ProbabilisticGrammar>,
   pub satisfied_rules: Vec<i64>,
}

impl ParseLine {
   pub fn open_rules(&self) -> Vec<i64> {
      let mut open = Vec::new();
      for r in self.satisfied_rules.iter() {
         if *r==0 { /* pass */ }
         else if *r>0 { 
            open.push(*r);
         } else {
            assert!(open.pop() == Some(-r));
         }
      }
      open
   }

   pub fn passed_tokens(&self) -> usize {
      self.satisfied_rules.iter().filter(|&n| *n == 0).count()
   }

   pub fn poke(&self, cs: &[char]) -> Vec<ParseLine> {
      assert!(self.satisfied_rules.len()>0);
      if self.satisfied_rules.last()==Some(&-1) {
         //ROOT is closed
         Vec::new()
      } else {
         let mut acc = Vec::new();
         let open = self.open_rules();
         let passed = self.passed_tokens();
         let active = *open.last().unwrap();
         match self.grammar.lookup(active).as_ref() {
            //return expanded parseline from open ruleset
            GrammarRule::Node(_id,_name,_node) => {},
            GrammarRule::Seq(_id,_name,_rule) => {},
            GrammarRule::Any(_id,_name,_rule) => {},
         }
         acc
      }
   }

   pub fn probability(&self) -> f64 {
      1.0
   }
}

pub struct GrammarNode {
   pub accepts: Box<dyn Fn(&Vec<char>) -> bool>,
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

   //This is the grammar starting from ROOT
   grammar_rules: Option<Rc<GrammarRule>>,

   //This is used for calculating posteriors, which are flattened into a DFA graph
   //This is not for parsing directly, because parsing is still Context Free
   //In order to fit into array indices, the grammar rule transitions are encoded as follows
   //   3n-3  is  continue rule N
   //   3n-2  is  open rule N
   //   3n-1  is  close rule N
   grammar_tensor: Rc<Array2<f64>>,

   //This index is for quick retrieval of grammar rules during parsing
   grammar_index: Rc<Vec<Rc<GrammarRule>>>,
}

impl Default for ProbabilisticGrammar {
    fn default() -> Self {
       ProbabilisticGrammar {
          dropdown_penalty: 0.9,
          max_lines: 10_000,
          grammar_rules: None,
          grammar_tensor: Rc::new(Array2::<f64>::zeros((0,0))),
          grammar_index: Rc::new(Vec::new()),
       }
    }
}

impl ProbabilisticGrammar {
   pub fn lookup(&self, i: i64) -> Rc<GrammarRule> {
      assert!(i>0 && i<(self.grammar_index.len() as i64)+1);
      self.grammar_index[(i-1) as usize].clone()
   }

   pub fn load<P: AsRef<std::path::Path>>(_encf: P) -> Self {
      Default::default()
   }

   pub fn train<P: AsRef<std::path::Path>>(&self, _dat: P) {} 

   pub fn recognize(&self, cs: &str) -> ParseResult {
      let rcs = Rc::new(self.clone());
      let mut lines: Vec<ParseLine> = if let Some(_) = &self.grammar_rules {
         vec![ ParseLine {
            grammar: rcs.clone(),
            satisfied_rules: vec![1],
         } ]
      } else { vec![] };
      let ccs = cs.chars().collect::<Vec<char>>();
      for ci in 1..ccs.len()+1 {
         let mut new_lines: Vec<ParseLine> = Vec::new();
         for l in lines.iter() {
            new_lines.append(&mut l.poke(&ccs[..ci]));
         }
         new_lines.sort_by(|a,b| a.probability().partial_cmp(&b.probability()).unwrap() );
         new_lines.reverse();
         new_lines.truncate(self.max_lines);
         lines = new_lines;
      }
      ParseResult {
         grammar: rcs.clone(),
         result: lines
      }
   }
}

