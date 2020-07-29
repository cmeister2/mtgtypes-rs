#![deny(missing_docs)]

//! Module for types in Magic: the Gathering

/// TODO
pub struct ManaCost {

}

/// TODO
pub struct ColorIndicator {

}
/// 205.2a The card types are artifact, conspiracy, creature, enchantment, instant,
/// land, phenomenon, plane, planeswalker, scheme, sorcery, tribal, and vanguard.
/// See section 3, “Card Types.”
#[allow(missing_docs)]
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

    /// For custom implementations of cards, a Custom option is provided.
    Custom(String)
}

/// 205.4a A card can also have one or more supertypes. These are printed directly
/// before its card types. The supertypes are basic, legendary, ongoing, snow, and
/// world.
pub enum SuperType {
    /// 205.4c Any land with the supertype “basic” is a basic land. Any land that
    ///        doesn’t have this supertype is a nonbasic land, even if it has a
    ///        basic land type. Cards printed in sets prior to the Eighth Edition
    ///        core set didn’t use the word “basic” to indicate a basic land. Cards
    ///        from those sets with the following names are basic lands and have
    ///        received errata in the Oracle card reference accordingly: Forest,
    ///        Island, Mountain, Plains, Swamp, Snow-Covered Forest,
    ///        Snow-Covered Island, Snow-Covered Mountain, Snow-Covered Plains,
    ///        and Snow-Covered Swamp.
    Basic,

    /// 205.4d Any permanent with the supertype “legendary” is subject to the
    ///        state-based action for legendary permanents, also called the
    ///        “legend rule” (see rule 704.5j).
    /// 205.4e Any instant or sorcery spell with the supertype “legendary” is
    ///        subject to a casting restriction. A player can’t cast a legendary
    ///        instant or sorcery spell unless that player controls a legendary
    ///        creature or a legendary planeswalker.
    Legendary,

    /// 205.4h Any scheme card with the supertype “ongoing” is exempt from the
    ///        state-based action for schemes (see rule 704.6e).
    Ongoing,

    /// 205.4g Any permanent with the supertype “snow” is a snow permanent. Any
    ///        permanent that doesn’t have this supertype is a nonsnow permanent,
    ///        regardless of its name.
    Snow,

    /// 205.4f Any permanent with the supertype “world” is subject to the
    ///        state-based action for world permanents, also called the “world rule”
    ///        (see rule 704.5k).
    World,

    /// A customizable string for custom cards.
    Custom(String)
}

/// TODO
#[allow(missing_docs)]
pub enum SubType {
    Custom(String)
}

/// 205.1. The type line is printed directly below the illustration. It contains the
/// card’s card type(s). It also contains the card’s subtype(s) and supertype(s), if
/// applicable.
pub struct TypeLine {
    /// 205.2a The card types are artifact, conspiracy, creature, enchantment,
    ///        instant, land, phenomenon, plane, planeswalker, scheme, sorcery,
    ///        tribal, and vanguard. See section 3, “Card Types.”
    /// 205.2b Some objects have more than one card type (for example, an artifact
    ///        creature). Such objects satisfy the criteria for any effect that applies
    ///        to any of their card types.
    pub card_type: Vec<CardType>,

    /// 205.4a A card can also have one or more supertypes. These are printed directly
    ///        before its card types. The supertypes are basic, legendary, ongoing,
    ///        snow, and world.
    pub super_type: Vec<SuperType>,

    /// 205.3a A card can have one or more subtypes printed on its type line.
    pub sub_type: Vec<SubType>,
}

/// 200.1. The parts of a card are name, mana cost, illustration, color indicator,
/// type line, expansion symbol, text box, power and toughness, loyalty, hand modifier,
/// life modifier, illustration credit, legal text, and collector number. Some cards
/// may have more than one of any or all of these parts.
pub trait Card {
    /// Gets the card name.
    fn name(&self) -> String;

    /// Gets the card mana cost, if it has one.
    fn mana_cost(&self) -> Option<ManaCost>;

    /// Gets the card's illustration, if there is one.
    fn illustration(&self) -> Option<String>;

    /// Gets the card's color indicator, if it has one.
    fn color_indicator(&self) -> Option<ColorIndicator>;

    /// Gets the card's type line.
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