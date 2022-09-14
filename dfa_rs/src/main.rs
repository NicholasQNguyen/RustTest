use crate::dfa::DFA;
use crate::dfa::Rule;
mod dfa;

fn main() {
    let test_rule = Rule{state: 1,
                         symbol:  "Hello".to_string(),
                         state_2: 2};
    let rule_array: [Rule; 1] = [test_rule; 1];
    let dfa1 = DFA::new([1].to_vec(),
                        "Hello".to_string(),
                        rule_array,
                        1,
                        [2].to_vec());
    println!("{}", dfa1);
    // Will panic if don't pass unit tests
    assert_eq!(true, dfa1.accept("Yes".to_string()));
}
