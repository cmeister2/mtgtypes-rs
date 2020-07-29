//! Module for types in Magic: the Gathering

// TODO
pub struct ManaCost {

}

// TODO
pub struct ColorIndicator {

}

// 205.2a The card types are artifact, conspiracy, creature, enchantment, instant,
// land, phenomenon, plane, planeswalker, scheme, sorcery, tribal, and vanguard.
// See section 3, “Card Types.”
pub enum CardType {
    Artifact,
    Conspiracy,
    Creature,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,

    // For custom implementations of cards, a Custom option is provided.
    Custom(String)
}

// 205.4a A card can also have one or more supertypes. These are printed directly
// before its card types. The supertypes are basic, legendary, ongoing, snow, and
// world.
pub enum SuperType {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
    Custom(String)
}

// TODO
pub enum SubType {
    Custom(String)
}

// 205.1. The type line is printed directly below the illustration. It contains the
// card’s card type(s). It also contains the card’s subtype(s) and supertype(s), if
// applicable.
pub struct TypeLine {
    pub card_type: Vec<CardType>,
    pub super_type: Vec<SuperType>,
    pub sub_type: Vec<SubType>,
}

// 200.1. The parts of a card are name, mana cost, illustration, color indicator,
// type line, expansion symbol, text box, power and toughness, loyalty, hand modifier,
// life modifier, illustration credit, legal text, and collector number. Some cards
// may have more than one of any or all of these parts.
pub trait Card {
    // Gets the card name.
    fn name(&self) -> String;

    // Gets the card mana cost, if it has one.
    fn mana_cost(&self) -> Option<ManaCost>;

    // Gets the card's illustration, if there is one.
    fn illustration(&self) -> Option<String>;

    // Gets the card's color indicator, if it has one.
    fn color_indicator(&self) -> Option<ColorIndicator>;

    // Gets the card's type line.
    fn type_line(&self) -> TypeLine;

    // Gets the card's expansion symbol
    // TODO

    // Gets the card's text box
    // TODO

    // Gets the card's power
    // TODO

    // Gets the card's toughness
    // TODO

    // Gets the card's loyalty
    // TODO

    // Gets the card's hand modifier
    // TODO

    // Gets the card's life modifier
    // TODO

    // Gets the card's illustration credit
    // TODO

    // Gets the card's legal text
    // TODO

    // Gets the card's collector number
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestCard {}

    impl Card for TestCard {
        fn name(&self) -> String {
            "TestCard".to_string()
        }
        fn mana_cost(&self) -> Option<ManaCost> {
            None
        }
        fn illustration(&self) -> Option<String> {
            None
        }
        fn color_indicator(&self) -> Option<ColorIndicator> {
            None
        }
        fn type_line(&self) -> TypeLine {
            TypeLine {
                card_type: vec!(CardType::Custom("TestCard".to_string())),
                super_type: vec!(),
                sub_type: vec!(),
            }
        }
    }

    #[test]
    fn test_card_name() {
        let tc = TestCard::default();
        assert_eq!(tc.name(), "TestCard");
    }
}