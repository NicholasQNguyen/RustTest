#[derive(Debug, PartialEq)]
pub struct Rule {
    pub state: i32,
    pub symbol: String,
    pub state_2: i32
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[State: {}, Symbol: {}, State 2: {}]", self.state, self.symbol, self.state_2)
    }
}

pub struct DFA {
    pub states: Vec<i32>,
    pub alphabet: String,
    pub transition_function: [Rule; 1],
    pub start_state: i32,
    pub acceptable_states: Vec<i32>
}

impl DFA {
    // Constructor Method
    pub fn new(states: Vec<i32>,
               alphabet: String,
               transition_function: [Rule; 1],
               start_state: i32,
               acceptable_states: Vec<i32>) -> Self {
        Self {states: states,
              alphabet: alphabet,
              transition_function: transition_function,
              start_state: start_state,
              acceptable_states: acceptable_states}
    }

    // Returns true is DFA accepts the string
    pub fn accept(&self, question: String) -> bool {
        for state in self.acceptable_states.iter() {
            if question.trim().parse::<i32>().unwrap() == *state {
                return true;
            }
         }

        return false;
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
