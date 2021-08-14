use super::rotor::Rotor;

#[allow(non_camel_case_types, dead_code)]
pub enum ReflectorType {
    UKW_A,
    TEST,
}

impl ReflectorType {
    fn value(&self) -> Rotor {
        match *self {
            ReflectorType::UKW_A => Rotor::with_key("EJMZALYXVBWFCRQUONTSPIKHGD"),
            ReflectorType::TEST => Rotor::with_key("ABC"),
        }
    }
}

#[allow(dead_code)]
pub enum RotorType {
    I,
    TEST,
    TEST_NOTCHLESS,
}

impl RotorType {
    fn value(&self) -> Rotor {
        match *self {
            RotorType::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Y'),
            RotorType::TEST => Rotor::new("ABC", 'B'),
            RotorType::TEST_NOTCHLESS => Rotor::with_key("ABC"),
        }
    }
}

pub struct RotorManager {
    pub(super) rotors: Vec<Rotor>,
    reflector: Rotor,
}

impl RotorManager {
    pub fn new(reflector_type: ReflectorType) -> Self {
        Self {
            rotors: Vec::new(),
            reflector: reflector_type.value(),
        }
    }

    pub fn add_rotor(mut self, rotor_type: RotorType) -> Self {
        self.rotors.push(rotor_type.value());
        self
    }

    pub fn update_rotors(&mut self) {
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
    pub fn should_add_rotor() {
        let rotor_manager = RotorManager::new(ReflectorType::TEST).add_rotor(RotorType::TEST);
        assert_eq!(rotor_manager.rotors.len(), 1)
    }

    #[test]
    pub fn should_rotate_first_but_not_second() {
        let mut rotor_manager = RotorManager::new(ReflectorType::TEST)
            .add_rotor(RotorType::TEST)
            .add_rotor(RotorType::TEST);

        rotor_manager.update_rotors();

        let mut rotors_iter = rotor_manager.rotors.iter();
        let first_rotor = rotors_iter.next().unwrap();
        let second_rotor = rotors_iter.next().unwrap();

        assert_eq!(first_rotor.state.get(0).unwrap(), &'B');
        assert_eq!(second_rotor.state.get(0).unwrap(), &'A');
    }

    #[test]
    pub fn should_rotate_first_and_second() {
        let mut rotor_manager = RotorManager::new(ReflectorType::TEST)
            .add_rotor(RotorType::TEST)
            .add_rotor(RotorType::TEST);

        rotor_manager.update_rotors();
        rotor_manager.update_rotors();

        let mut rotors_iter = rotor_manager.rotors.iter();
        let first_rotor = rotors_iter.next().unwrap();
        let second_rotor = rotors_iter.next().unwrap();

        assert_eq!(first_rotor.state.get(0).unwrap(), &'C');
        assert_eq!(second_rotor.state.get(0).unwrap(), &'B');
    }
}
