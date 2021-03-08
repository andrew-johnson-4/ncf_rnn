use std::rc::Rc;
use ncf_rnn::*;

#[test]
fn parse1() {
   assert!(ProbabilisticGrammar {
      grammar_rules: Some(Rc::new(GrammarRule::Node(1,"ROOT".to_string(),Rc::new(GrammarNode {
         accepts: Box::new(|cs| cs.iter().all(|&c| c=='a')),
      })))),
      ..Default::default()
   }.recognize("").probability() > 0.5);

   assert!(ProbabilisticGrammar {
      grammar_rules: Some(Rc::new(GrammarRule::Node(1,"ROOT".to_string(),Rc::new(GrammarNode {
         accepts: Box::new(|cs| cs.iter().all(|&c| c=='a')),
      })))),
      ..Default::default()
   }.recognize("a").probability() > 0.5);

   assert!(ProbabilisticGrammar {
      grammar_rules: Some(Rc::new(GrammarRule::Node(1,"ROOT".to_string(),Rc::new(GrammarNode {
         accepts: Box::new(|cs| cs.iter().all(|&c| c=='a')),
      })))),
      ..Default::default()
   }.recognize("b").probability() < 0.5);
}
