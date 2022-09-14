use crate::dfa::DFA;
use crate::dfa::Rule;
mod dfa;

fn main() {
    let test_rule = Rule{state: 1,
                         symbol:  "Hello".to_string(),
                         state_2: 2};
    let test_rule2 = Rule{state: 1,
                         symbol:  "Hello".to_string(),
                         state_2: 2};
    let rule_array: [Rule; 1] = [test_rule; 1];
    let rule_array2: [Rule; 1] = [test_rule2; 1];
    let dfa1 = DFA::new([1].to_vec(),
                        "Hello".to_string(),
                        rule_array,
                        1,
                        [2].to_vec());
    println!("{}", dfa1);

    // Unit Tests
    // Will panic if don't pass unit tests
    assert_eq!(true, dfa1.accept("2".to_string()));
    assert_eq!([1].to_vec(), dfa1.states);
    assert_eq!("Hello".to_string(), dfa1.alphabet);
    assert_eq!(1, dfa1.start_state);
    assert_eq!(rule_array2, dfa1.transition_function);
    assert_eq!([2].to_vec(), dfa1.acceptable_states);

    println!("Got through the tests, boss!");
}
