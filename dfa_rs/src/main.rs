use crate::dfa::DFA;
mod dfa;

fn main() {
    let dfa1 = DFA::new();
    println!("{}", dfa1);
    assert_eq!(true, dfa1.accept("Yes".to_string()));
}
