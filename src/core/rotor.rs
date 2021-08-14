use std::collections::VecDeque;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
pub struct Rotor {
    pub(super) state: VecDeque<char>,
    notch: Option<char>,
}

impl Rotor {
    pub fn new(set: &str, notch: char) -> Self {
        let mut state = VecDeque::with_capacity(set.len());
        for c in set.chars() {
            state.push_back(c);
        }

        let notch = if notch.is_whitespace() {
            None
        } else {
            Some(notch)
        };

        Self { state, notch }
    }

    pub fn with_set(set: &str) -> Self {
        Rotor::new(set, ' ')
    }

    pub fn encode(&self, value: &char) -> char {
        let value: String = value.to_uppercase().collect();
        println!("char to be encrypted: {}", value);
        println!("current rotor state: {:?}", self.state);
        let index = ALPHABET
            .chars()
            .position(|c| c == value.chars().nth(0).unwrap())
            .unwrap();
        let enc_char = self.state.get(index).unwrap().to_owned();
        println!("encrypted char: {}", enc_char);
        enc_char
    }

    pub fn rotate(&mut self) {
        let first = self.state.pop_front().unwrap();
        self.state.push_back(first);
    }

    pub fn at_notch(&self) -> bool {
        let value = self.state.get(0).unwrap();
        if let Some(notch) = self.notch.as_ref() {
            if value == notch {
                return true;
            } else {
                return false;
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn enc_lowercase() {
        let rotor = Rotor::with_set("CBA");
        assert_eq!('C', rotor.encode(&'a'));
    }

    #[test]
    pub fn enc_uppercase() {
        let rotor = Rotor::with_set("CBA");
        assert_eq!('C', rotor.encode(&'A'));
    }

    #[test]
    pub fn rotate() {
        let mut rotor = Rotor::with_set("ABC");
        rotor.rotate();
        assert_eq!(rotor.encode(&'a'), 'B');
    }

    #[test]
    pub fn at_notch_when_has_one() {
        let rotor = Rotor::new("ABC", 'A');
        assert!(rotor.at_notch());
    }

    #[test]
    pub fn not_at_notch_when_doesnt_have_one() {
        let rotor = Rotor::with_set("ABC");
        assert!(!rotor.at_notch());
    }

    #[test]
    pub fn not_at_notch_when_has_one() {
        let rotor = Rotor::new("ABC", 'B');
        assert!(!rotor.at_notch());
    }
}
