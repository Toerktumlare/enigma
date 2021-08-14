use super::rotor::Rotor;
use super::rotor_manager::{ReflectorType, RotorManager, RotorType};

pub struct Enigma {
    rotor_manager: RotorManager,
}

impl Enigma {
    pub fn new() -> Self {
        let rotor_manager = RotorManager::new(ReflectorType::TEST)
            .add_rotor(RotorType::TEST)
            .add_rotor(RotorType::TEST);
        Self { rotor_manager }
    }

    pub fn with_rotor_manager(rotor_manager: RotorManager) -> Self {
        Self { rotor_manager }
    }

    pub fn encrypt(&mut self, value: &str) -> String {
        let mut value = String::from(value);
        value.retain(|c| !c.is_whitespace());
        let mut enc_string = String::with_capacity(value.len());

        self.rotor_manager.update_rotors();

        for c in value.chars() {
            let mut c_under_enc = c;
            for rotor in self.rotor_manager.rotors.iter_mut().peekable() {
                c_under_enc = rotor.encode(&c_under_enc);
            }
            enc_string.push(c_under_enc);
        }
        enc_string
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn enigma_one_rotor_with_rotation() {
        let rotor_manager = RotorManager::new(ReflectorType::TEST).add_rotor(RotorType::TEST);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "B");
    }

    #[test]
    pub fn enigma_two_rotors_no_notch() {
        let rotor_manager = RotorManager::new(ReflectorType::TEST)
            .add_rotor(RotorType::TEST_NOTCHLESS)
            .add_rotor(RotorType::TEST_NOTCHLESS);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "B");
    }

    #[test]
    pub fn if_at_notch_should_rotate_second_rotor() {
        let rotor_manager = RotorManager::new(ReflectorType::TEST)
            .add_rotor(RotorType::TEST)
            .add_rotor(RotorType::TEST_NOTCHLESS);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "B");
    }
}
