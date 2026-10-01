//! Offline name offer from two fixed lists.
//!
//! Cite `docs/NAME_RITE.md`: offline, no model, assist-used flag untouched.

pub const GIVEN: &[&str] = &[
    "Calwen", "Delwen", "Elsiv", "Galme", "Hespel", "Jorven", "Kelvi", "Lirren", "Orven", "Pellon",
    "Tavin", "Virel",
];

pub const HOUSE: &[&str] = &[
    "Ashfen", "Cormal", "Dunwel", "Fenlow", "Ithrel", "Jastel", "Ostlen", "Pellin", "Sennel",
    "Solwen", "Velden", "Wynholt",
    "Vysholt",
];

pub fn offer(seed: u64) -> String {
    format!(
        "{} {}",
        GIVEN[(seed % GIVEN.len() as u64) as usize],
        HOUSE[(seed % HOUSE.len() as u64) as usize]
    )
}

pub fn refuse(seed: u64) -> u64 {
    seed.wrapping_add(1)
}

pub fn typed_is_refuse(typed: &str) -> bool {
    typed.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_offer() {
        let seed = 41u64;
        assert_eq!(offer(seed), offer(seed));
        assert_eq!(
            offer(seed),
            format!(
                "{} {}",
                GIVEN[(seed % GIVEN.len() as u64) as usize],
                HOUSE[(seed % HOUSE.len() as u64) as usize]
            )
        );
    }

    #[test]
    fn refuse_steps_both_list_indexes() {
        for seed in [0u64, 1, 11, 12, 100, u64::MAX - 1] {
            let stepped = refuse(seed);
            let given_now = (seed % GIVEN.len() as u64) as usize;
            let house_now = (seed % HOUSE.len() as u64) as usize;
            let given_next = (stepped % GIVEN.len() as u64) as usize;
            let house_next = (stepped % HOUSE.len() as u64) as usize;
            assert_eq!(given_next, (given_now + 1) % GIVEN.len());
            assert_eq!(house_next, (house_now + 1) % HOUSE.len());
            assert_eq!(
                offer(stepped),
                format!("{} {}", GIVEN[given_next], HOUSE[house_next])
            );
        }
    }

    #[test]
    fn empty_or_whitespace_typed_is_refuse() {
        assert!(typed_is_refuse(""));
        assert!(typed_is_refuse(" "));
        assert!(typed_is_refuse("\t"));
        assert!(typed_is_refuse("\n"));
        assert!(typed_is_refuse(" \t\n "));
        assert!(!typed_is_refuse("Virel"));
        assert!(!typed_is_refuse(" Virel "));
        assert!(!typed_is_refuse("a"));
    }

    #[test]
    fn lists_are_short_capital_ascii_words() {
        assert_word_list(GIVEN);
        assert_word_list(HOUSE);
    }

    #[test]
    fn refuse_wraps_u64_max_and_offer_max_holds() {
        assert_eq!(refuse(u64::MAX), 0);
        let offered = offer(u64::MAX);
        assert_eq!(
            offered,
            format!(
                "{} {}",
                GIVEN[(u64::MAX % GIVEN.len() as u64) as usize],
                HOUSE[(u64::MAX % HOUSE.len() as u64) as usize]
            )
        );
    }

    #[test]
    fn given_and_house_lengths_are_coprime() {
        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }
        assert_eq!(gcd(GIVEN.len(), HOUSE.len()), 1);
    }

    #[test]
    fn offer_span_yields_156_distinct_strings() {
        use std::collections::HashSet;
        let span = GIVEN.len() * HOUSE.len();
        let offers: HashSet<String> = (0..span as u64).map(offer).collect();
        assert_eq!(offers.len(), 156);
    }

    fn assert_word_list(words: &[&str]) {
        assert!((8..=16).contains(&words.len()));
        for (i, word) in words.iter().enumerate() {
            assert!((3..=8).contains(&word.len()), "{word}");
            assert!(word.chars().all(|c| c.is_ascii_alphabetic()), "{word}");
            let mut chars = word.chars();
            let first = chars.next().expect("length already checked");
            assert!(first.is_ascii_uppercase(), "{word}");
            assert!(chars.all(|c| c.is_ascii_lowercase()), "{word}");
            for other in words.iter().skip(i + 1) {
                assert_ne!(word, other, "duplicate {word}");
            }
        }
    }
}
