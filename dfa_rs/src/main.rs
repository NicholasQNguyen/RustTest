use crate::dfa::DFA;
use crate::dfa::Rule;
mod dfa;

fn init() -> DFA {
    let test_rule = Rule{state: 1,
                         symbol:  "Hello".to_string(),
                         state_2: 2};
    let rule_array: [Rule; 1] = [test_rule; 1];
    DFA::new([1].to_vec(),
             "Hello".to_string(),
              rule_array,
              1,
              [2].to_vec())
}

fn main() {
    println!("Got through the tests, boss!");
}

#[test]
fn test_accept_true () {
    let dfa = init();
    assert_eq!(true, dfa.accept("2".to_string()));
}

#[test]
fn test_accept_false () {
    let dfa = init();
    assert_eq!(false, dfa.accept("1".to_string()));
}

#[test]
fn test_dfa_states () {
    let dfa = init();
    assert_eq!([1].to_vec(), dfa.states);
}

#[test]
fn test_dfa_alphabet () {
    let dfa = init();
    assert_eq!("Hello".to_string(), dfa.alphabet);
}

#[test]
fn test_dfa_transition_function () {
    let dfa = init();
    let test_rule2 = Rule{state: 1,
                         symbol:  "Hello".to_string(),
                         state_2: 2};
    let rule_array2: [Rule; 1] = [test_rule2; 1];
    assert_eq!(rule_array2, dfa.transition_function);
}

#[test]
fn test_dfa_start_state () {
    let dfa = init();
    assert_eq!(1, dfa.start_state);
}

#[test]
fn test_dfa_acceptable_state () {
    let dfa = init();
    assert_eq!([2].to_vec(), dfa.acceptable_states);
}
