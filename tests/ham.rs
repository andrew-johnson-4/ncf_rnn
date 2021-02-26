
#[test]
fn ham_untrained() {
   let ham = ncf_rnn::load_grammar("ham.encf");
   assert!( ham.recognize("Hello World, I am so pleased to meet you!").probability() > 0.5 );
   assert!( ham.recognize("Go away you dirty turd face.").probability() < 0.5 );
}

#[test]
fn ham_trained() {
   let ham = ncf_rnn::load_grammar("ham.encf");
   ham.train("ham_corpus.txt");
   assert!( ham.recognize("Hello World, I am so pleased to meet you!").probability() > 0.5 );
   assert!( ham.recognize("Go away you dirty turd face.").probability() < 0.5 );
}
