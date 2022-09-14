#[derive(Debug)]
pub struct Rule {
    state: i32,
    symbol: String,
    state_2: i32
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[State: {}, Symbol: {}, State 2: {}]", self.state, self.symbol, self.state_2)
    }
}

pub struct DFA {
    states: Vec<i32>,
    alphabet: String,
    transition_function: Vec<Rule>,
    start_state: i32,
    acceptable_states: Vec<i32>
}

impl DFA {
    // Constructor Method
    pub fn new() -> Self {
        let test_vec = vec!(0);
        let test_vec2 = vec!(1);
        let test_rule = Rule{state: 1,
                             symbol:  "Hello".to_string(),
                             state_2: 2};
        let rule_vec = vec!(test_rule);
        Self {states: test_vec,
              alphabet: "Hello".to_string(),
              transition_function: rule_vec,
              start_state: 1,
              acceptable_states: test_vec2}
    }

    // Returns true is DFA accepts the string
    pub fn accept(&self, question: String) -> bool {
        if question.trim() == "Yes" {
            true
        }
        else {
            false
        }
    }
}

// Fancy formatting for printing
impl std::fmt::Display for DFA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
               "States: {:?} \nAlphabet: {}\nTransition Function: {}\nStart State: {}\nAcceptable States: {:?}",
               self.states, self.alphabet, self.transition_function[0], self.start_state, self.acceptable_states)
    }
}
