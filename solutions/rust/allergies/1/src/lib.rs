pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => self.score & (1 << 0) != 0,
            Allergen::Peanuts => self.score & (1 << 1) != 0,
            Allergen::Shellfish => self.score & (1 << 2) != 0,
            Allergen::Strawberries => self.score & (1 << 3) != 0,
            Allergen::Tomatoes => self.score & (1 << 4) != 0,
            Allergen::Chocolate => self.score & (1 << 5) != 0,
            Allergen::Pollen => self.score & (1 << 6) != 0,
            Allergen::Cats => self.score & (1 << 7) != 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut list = Vec::new();

        let all_allergens = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        for (bit_index, allergen) in all_allergens.into_iter().enumerate() {
            if self.score & (1 << bit_index) != 0 {
                list.push(allergen);
            }
        }

        list
    }
}
