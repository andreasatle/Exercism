//! Given a person's allergy score, determine whether or not they're allergic to a given item,
//! and their full list of allergies.
//! 
//! An allergy test produces a single numeric score which contains the information about all
//! the allergies the person has (that they were tested for).
//! 
//! The list of items (and their value) that were tested are:
//! 
//! * eggs (1)
//! * peanuts (2)
//! * shellfish (4)
//! * strawberries (8)
//! * tomatoes (16)
//! * chocolate (32)
//! * pollen (64)
//! * cats (128)
//! 
//! So if Tom is allergic to peanuts and chocolate, he gets a score of 34.
//! 
//! Now, given just that score of 34, your program should be able to say:
//! 
//! Whether Tom is allergic to any one of those allergens listed above.
//! All the allergens Tom is allergic to.
//! Note: a given score may include allergens not listed above (i.e. allergens that
//! score 256, 512, 1024, etc.). Your program should ignore those components of the
//! score. For example, if the allergy score is 257, your program should only report
//! the eggs (1) allergy.

/// Representation of the different allergies.
pub struct Allergies {
    score: u32,
}

/// Definition of the different allergies.
/// Remark: Clone and Copy are necessary to use the enum as an u32 (of some reason).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    /// Create a new instance of Allergies.
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    /// Check if allergen is contained in Allergies.
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (*allergen as u32) != 0
    }

    /// List all the allergies of the current Allergies.
    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens_with_masks = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
            ];

            let mut allergens = Vec::new();
            for allergen in allergens_with_masks {
            if self.score & (allergen as u32) != 0 {
                allergens.push(allergen)
            }    
        }

        allergens
    }
}
