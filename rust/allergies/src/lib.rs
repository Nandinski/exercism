pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl From<u32> for Allergen {
    fn from(value: u32) -> Allergen {
        match value {
            0 => Allergen::Eggs,
            1 => Allergen::Peanuts,
            2 => Allergen::Shellfish,
            3 => Allergen::Strawberries,
            4 => Allergen::Tomatoes,
            5 => Allergen::Chocolate,
            6 => Allergen::Pollen,
            7 => Allergen::Cats,
            _ => panic!("Invalid allergen index"),
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergies: Self::calculate_allergies(score),
        }
    }

    fn calculate_allergies(score: u32) -> Vec<Allergen> {
        let mut allergies = Vec::new();

        for bit_pos in 0..8 {
            if score & (1 << bit_pos) != 0 {
                allergies.push(Allergen::from(bit_pos));
            }
        }

        allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
