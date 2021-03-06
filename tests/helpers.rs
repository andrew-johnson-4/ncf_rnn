use std::rc::Rc;
use ncf_rnn::ParseLine;

#[test]
fn parse_line_open_rules() {
    assert!(ParseLine {
       grammar: Rc::new(Default::default()),
       satisfied_rules: vec![1],
    }.open_rules() == vec![1]);

    assert!(ParseLine {
       grammar: Rc::new(Default::default()),
       satisfied_rules: vec![1,2,0],
    }.open_rules() == vec![1,2]);

    assert!(ParseLine {
       grammar: Rc::new(Default::default()),
       satisfied_rules: vec![1,2,0,-2,3],
    }.open_rules() == vec![1,3]);
}
