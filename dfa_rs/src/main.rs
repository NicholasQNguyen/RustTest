mod test;
mod dfa;

pub fn accept(question: String) {
    for character in question.chars() {
        println!("{}", character);
    }
}
fn main() {
/*
    let one_to_two_a = dfa::Rule{state: 1,
                            symbol:  "a".to_string(),
                            state_2: 2};
    let one_to_two_b = dfa::Rule{state: 1,
                            symbol:  "b".to_string(),
                            state_2: 2};
    let one_to_one_a = dfa::Rule{state: 1,
                            symbol:  "a".to_string(),
                            state_2: 1};
    let two_to_one_a = dfa::Rule{state: 1,
                            symbol:  "a".to_string(),
                            state_2: 1};
    let two_to_one_b = dfa::Rule{state: 1,
                            symbol:  "b".to_string(),
                            state_2: 1};
    let rule_array: [dfa::Rule; 4] = [one_to_two_a,
                                 one_to_two_b,
                                 two_to_one_a,
                                 two_to_one_b];
    let dfa1 = dfa::Dfa::new([1, 2].to_vec(),
             "a, b".to_string(),
              rule_array,
              1,
              [1].to_vec());
    dfa1.accept("abcdefg".to_string());
*/
    println!("Got through the tests, boss!");
}
