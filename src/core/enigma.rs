use super::rotor::Rotor;

pub struct Enigma {
    rotors: Vec<Rotor>,
}

impl Enigma {
    pub fn new() -> Self {
        let set = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let rotor = Rotor::new(set, 'k');
        let rotors = vec![rotor];
        Self { rotors }
    }

    pub fn with_rotors(rotors: Vec<Rotor>) -> Self {
        Self { rotors }
    }

    pub fn encrypt(&mut self, value: &str) -> String {
        let mut value = String::from(value);
        value.retain(|c| !c.is_whitespace());
        let mut enc_string = String::with_capacity(value.len());

        // update rotors into position
        self.update_rotors();
        // make the connection

        for c in value.chars() {
            let mut c_under_enc = c;
            for rotor in self.rotors.iter_mut().peekable() {
                c_under_enc = rotor.encode(&c_under_enc);
            }
            enc_string.push(c_under_enc);
        }
        enc_string
    }

    fn update_rotors(&mut self) {
        let mut rotate_next = true;
        for rotor in self.rotors.iter_mut() {
            let at_notch = rotor.at_notch();
            if rotate_next {
                rotor.rotate();
            }

            if !at_notch {
                rotate_next = false;
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn enigma_one_rotor_with_rotation() {
        let rotors = vec![Rotor::with_key("ABC")];
        let mut enigma = Enigma::with_rotors(rotors);
        assert_eq!("B", enigma.encrypt("a"));
    }

    #[test]
    pub fn enigma_two_rotors_no_notch() {
        let rotors = vec![Rotor::with_key("ABC"), Rotor::with_key("ABC")];
        let mut enigma = Enigma::with_rotors(rotors);
        assert_eq!(enigma.encrypt("a"), "B");
    }

    #[test]
    pub fn if_at_notch_should_rotate_second_rotor() {
        let rotors = vec![Rotor::new("ABC", 'A'), Rotor::with_key("ABC")];
        let mut enigma = Enigma::with_rotors(rotors);
        enigma.update_rotors();
        println!("{:?}", enigma.rotors);
    }

    #[test]
    pub fn should_rotate_first_rotor_not_second() {
        let rotors = vec![Rotor::new("ABC", 'B'), Rotor::with_key("ABC")];
        let mut enigma = Enigma::with_rotors(rotors);

        enigma.update_rotors();

        let mut rotor_iter = enigma.rotors.iter_mut();
        let first_rotor = rotor_iter.next().unwrap();
        let second_rotor = rotor_iter.next().unwrap();

        assert_eq!(first_rotor.state.pop_front(), Some('B'));
        assert_eq!(second_rotor.state.pop_front(), Some('A'));
    }
}
