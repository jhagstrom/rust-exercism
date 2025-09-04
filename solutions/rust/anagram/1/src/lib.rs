use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    let normalized_word = word.to_lowercase();
    let canonical_word: String = canonical_form(&normalized_word);
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for &w in possible_anagrams {
        let normalized_candidate = w.to_lowercase();
        if normalized_word != normalized_candidate 
            && canonical_form(&normalized_candidate) == canonical_word {
            anagrams.insert(w);
        }
    }
    anagrams
}

fn canonical_form(s: &str) -> String {
    let mut chars: Vec<char> = s
        .to_lowercase()
        .chars()
        .collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
