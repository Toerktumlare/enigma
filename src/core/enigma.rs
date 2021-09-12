use super::rotor_manager::{ReflectorType, RotorManager, RotorType};

pub struct Enigma {
    rotor_manager: RotorManager,
}

impl Enigma {
    pub fn new() -> Self {
        let rotor_manager = RotorManager::new(ReflectorType::UKW_A)
            .add_rotor(RotorType::I)
            .add_rotor(RotorType::I);
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

        for mut c in value.chars() {
            for rotor in self.rotor_manager.rotors.iter_mut().peekable() {
                c = rotor.encode(&c);
            }

            if let Some(reflector) = &self.rotor_manager.reflector {
                c = reflector.encode(&c);
            }

            for rotor in self.rotor_manager.rotors.iter_mut().rev().peekable() {
                c = rotor.encode(&c);
            }
            enc_string.push(c);
        }

        enc_string
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // One rotator no reflector
    //
    // A -> B:C:A = B
    // B -> B:C:A = C
    #[test]
    pub fn enigma_one_rotor_with_rotation() {
        let rotor_manager = RotorManager::new(ReflectorType::None).add_rotor(RotorType::Test);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "C");
    }

    // Two rotators no reflector
    //
    // A -> B:C:A = B
    // B -> A:B:C = B
    // B -> A:B:C = B
    // B -> B:C:A = C
    #[test]
    pub fn enigma_two_rotors_no_notch() {
        let rotor_manager = RotorManager::new(ReflectorType::None)
            .add_rotor(RotorType::Test)
            .add_rotor(RotorType::Test);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "C");
    }

    // Two rotators no reflector
    //
    // A -> B:C:A = B
    // B -> A:B:C = B
    // B -> A:B:C = B
    // B -> B:C:A = C
    //
    // A -> C:A:B = C
    // C -> B:C:A = A
    // A -> B:C:A = B
    // B -> C:A:B = A
    #[test]
    pub fn if_at_notch_should_rotate_second_rotor() {
        let rotor_manager = RotorManager::new(ReflectorType::Test)
            .add_rotor(RotorType::Test)
            .add_rotor(RotorType::TestNotchless);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        assert_eq!(enigma.encrypt("a"), "C");
        assert_eq!(enigma.encrypt("a"), "A");
    }

    // Each step through 2 rotors, reflector and then back
    // plain text: A
    //
    // Rotors input -> output
    // A -> B::C::A = B
    // B -> A::B::C = B
    //
    // Reflector
    // B -> A::B::C = B
    //
    // Rotors(reverse order) output -> input
    // B -> A::B::C = B
    // B -> B::C::A = C
    #[test]
    pub fn reflector() {
        let rotor_manager = RotorManager::new(ReflectorType::Test)
            .add_rotor(RotorType::Test)
            .add_rotor(RotorType::Test);
        let mut enigma = Enigma::with_rotor_manager(rotor_manager);
        let result = enigma.encrypt("A");
        assert_eq!(result, "C");
    }
}
