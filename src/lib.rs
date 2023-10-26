use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn head_tail(input: String) -> (Option<String>, String) {
    if input.is_empty() {
        return (None, input.to_string());
    }
    let mut graphemes = input.graphemes(true).into_iter(); //.collect::<Vec<&str>>();
    let head: Option<String> = graphemes.next().map(|s| s.to_string());

    let tail: String = match graphemes
        .map(|s| s.to_string())
        .reduce(|acc, s| format!("{}{}", acc, s))
    {
        Some(v) => v,
        None => "".to_string(),
    };
    (head, tail)
}

fn insert_sort<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> Vec<&T> {
    let list_to_pass: Vec<&T> = list.into_iter().collect();
    let sorted = insert_sort_rec_outer(list_to_pass.to_vec(), 1);
    sorted
}

fn insert_sort_rec_outer<T: std::cmp::PartialOrd + Copy>(list: Vec<&T>, i: usize) -> Vec<&T> {
    if i >= list.len() {
        return list.to_owned();
    }
    let new_list = insert_sort_rec_inner(list, i as i32);
    let new_i = i + 1;
    insert_sort_rec_outer(new_list, new_i)
}

fn insert_sort_rec_inner<T: std::cmp::PartialOrd + Copy>(list: Vec<&T>, j_input: i32) -> Vec<&T> {
    if j_input <= 0 {
        return list;
    }
    let j = j_input as usize;
    if !(list[j - 1] > list[j]) {
        return list;
    }
    let new_j: i32 = j as i32 - 1;
    let (j_el, j_minus1_el) = (list[j - 1], list[j]);
    let new_list: Vec<&T> = list
        .into_iter()
        .enumerate()
        .map(|(i, x)| match i {
            i if i == j => j_el,
            i if i == j - 1 => j_minus1_el,
            _ => x,
        })
        .collect();
    insert_sort_rec_inner(new_list, new_j)
}

fn sort_unicode_string(word: &str) -> String {
    match insert_sort(
        &word
            .to_lowercase()
            .graphemes(true)
            .into_iter()
            .collect::<Vec<&str>>(),
    )
    .into_iter()
    .map(|x| x.to_string())
    .reduce(|acc, x| format!("{}{}", acc, x))
    {
        Some(v) => v,
        None => "".to_owned(),
    }
}

fn are_words_anagrams(word: &str, another_word: &str) -> bool {
    if word.to_lowercase() == another_word.to_lowercase() {
        return false;
    }
    let sorted_word = sort_unicode_string(word);
    let sorted_another_word = sort_unicode_string(another_word);
    sorted_word == sorted_another_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&str> = HashSet::new();

    let anagrams_list = possible_anagrams
        .into_iter()
        .filter(|possible_anagram| are_words_anagrams(word, &possible_anagram));
    anagrams_list.for_each(|x| {
        anagrams.insert(x);
    });

    anagrams
}
