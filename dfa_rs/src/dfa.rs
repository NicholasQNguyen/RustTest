use std::char;

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    pub start_state: i32,
    pub symbol: char,
    pub end_state: i32
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[State: {}, Symbol: {}, State 2: {}]", self.start_state, self.symbol, self.end_state)
    }
}

pub struct Dfa {
    pub states: Vec<i32>,
    pub alphabet: String,
    pub transition_function: [Rule; 4],
    pub start_state: i32,
    pub acceptable_states: Vec<i32>
}

impl Dfa {
    // Constructor Method
    pub fn new(states: Vec<i32>,
               alphabet: String,
               transition_function: [Rule; 4],
               start_state: i32,
               acceptable_states: Vec<i32>) -> Self {
        Self {states,
              alphabet,
              transition_function,
              start_state,
              acceptable_states}
    }

    // Returns true is Dfa accepts the string
    pub fn accept(&self, question: String) -> bool {
        let mut current_state = self.start_state;
        for character in question.chars() {
            for i in 0..self.transition_function.len() {
                if character == self.transition_function[i].symbol {
                    if current_state == self.transition_function[i].start_state {
                        current_state = self.transition_function[i].end_state;
                        break;
                    }
                }
            }
        }
        for state in &self.acceptable_states {
            if current_state == *state {
                return true;
            }
        }
        false
    }
}
// Fancy formatting for printing
impl std::fmt::Display for Dfa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
               "States: {:?} \nAlphabet: {}\nTransition Function: {}\nStart State: {}\nAcceptable States: {:?}",
               self.states, self.alphabet, self.transition_function[0], self.start_state, self.acceptable_states)
    }
}
