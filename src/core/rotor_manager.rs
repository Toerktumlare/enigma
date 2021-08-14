use super::rotor::Rotor;

#[allow(non_camel_case_types)]
enum ReflectorType {
    UKW_A,
    TEST,
}

impl ReflectorType {
    fn value(&self) -> Rotor {
        match self {
            ReflectorType::UKW_A => Rotor::with_key("EJMZALYXVBWFCRQUONTSPIKHGD"),
            ReflectorType::TEST => Rotor::with_key("ABC"),
        }
    }
}

enum RotorType {
    I,
    TEST,
}

impl RotorType {
    fn value(&self) -> Rotor {
        match self {
            RotorType::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Y'),
            RotorType::TEST => Rotor::new("", 'Y'),
        }
    }
}

struct RotorManager {
    rotors: Vec<Rotor>,
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
}
