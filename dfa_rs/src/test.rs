#[cfg(test)]
use crate::dfa::Dfa;
use crate::dfa::Rule;


// Run this before each test to build the things we're testing
pub fn init() -> Dfa {
    // DFA that accepts strings of even length
    let one_to_two_a = Rule{state: 1,
                            symbol:  "a".to_string().chars().nth(0).unwrap(),
                            state_2: 2};
    let one_to_two_b = Rule{state: 1,
                            symbol:  "b".to_string().chars().nth(0).unwrap(),
                            state_2: 2};
    let two_to_one_a = Rule{state: 1,
                            symbol:  "a".to_string().chars().nth(0).unwrap(),
                            state_2: 1};
    let two_to_one_b = Rule{state: 1,
                            symbol:  "b".to_string().chars().nth(0).unwrap(),
                            state_2: 1};
    let rule_array: [Rule; 4] = [one_to_two_a,
                                 one_to_two_b,
                                 two_to_one_a,
                                 two_to_one_b];
    Dfa::new([1, 2].to_vec(),
             "a, b".to_string(),
              rule_array,
              1,
              [1].to_vec())
}

// Testing the accept method
#[test]
fn test_accept_true () {
    let dfa = init();
    assert_eq!(true, dfa.accept("aa".to_string()));
}

#[test]
fn test_accept_false () {
    let dfa = init();
    assert_eq!(false, dfa.accept("aaa".to_string()));
}


// Testing the struct contents
#[test]
fn test_dfa_states () {
    let dfa = init();
    assert_eq!([1, 2].to_vec(), dfa.states);
}

#[test]
fn test_dfa_alphabet () {
    let dfa = init();
    assert_eq!("a, b".to_string(), dfa.alphabet);
}

#[test]
fn test_dfa_transition_function () {
    let dfa = init();
    let one_to_two_a2 = Rule{state: 1,
                            symbol:  "a".to_string().chars().nth(0).unwrap(),
                            state_2: 2};
    let one_to_two_b2 = Rule{state: 1,
                            symbol:  "b".to_string().chars().nth(0).unwrap(),
                            state_2: 2};
    let two_to_one_a2 = Rule{state: 1,
                            symbol:  "a".to_string().chars().nth(0).unwrap(),
                            state_2: 1};
    let two_to_one_b2 = Rule{state: 1,
                            symbol:  "b".to_string().chars().nth(0).unwrap(),
                            state_2: 1};
    let compared_array: [Rule; 4] = [one_to_two_a2,
                                     one_to_two_b2,
                                     two_to_one_a2,
                                     two_to_one_b2];
    let rule_array2: [Rule; 4] = compared_array;
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
    assert_eq!([1].to_vec(), dfa.acceptable_states);
}
