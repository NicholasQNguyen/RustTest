#[derive(Debug)]
pub struct Rule {
    state: i32,
    symbol: String,
    state_2: i32
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[State: {}, Symbol: {}, State 2 {}]", self.state, self.symbol, self.state_2)
    }
}

// #[derive(Debug)]
pub struct DFA {
    states: Vec<i32>,
    alphabet: String,
    transition_function: Vec<Rule>,
    start_state: i32,
    acceptable_states: Vec<i32>
}

impl DFA {
    pub fn new() -> Self {
        let mut test_vec = Vec::new();
        test_vec.push(0);
        let mut test_vec2 = Vec::new();
        test_vec2.push(1);
        let test_rule = Rule{state: 1,
                             symbol:  "Hello".to_string(),
                             state_2: 2};
        let mut rule_vec = Vec::new();
        rule_vec.push(test_rule);
        Self {states: test_vec,
              alphabet: "Hello".to_string(),
              transition_function: rule_vec,
              start_state: 1,
              acceptable_states: test_vec2}
    }

    pub fn accept(&self, question: String) -> bool {
        if question.trim() == "Yes" {
            true
        }
        else {
            false
        }
    }
}

impl std::fmt::Display for DFA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
               "States: {:?} \nAlphabet: {}\nTransition Function: {}\nStart State: {}\nAcceptable States: {:?}",
               self.states, self.alphabet, self.transition_function[0], self.start_state, self.acceptable_states)
    }
}
