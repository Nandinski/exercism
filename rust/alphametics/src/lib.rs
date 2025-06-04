use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (lhs, rhs) = input.split_once(" == ").expect("expected == sign");
    let lhs: Vec<&str> = lhs.split(" + ").collect();

    let letter_factors_map = get_letter_factors(&lhs, rhs);
    let (letters, letter_factors): (Vec<char>, Vec<i64>) = letter_factors_map.iter().unzip();
    let leading_letter_indices = get_leading_letter_indices(&lhs, rhs, &letters);

    try_solve(&letters, &letter_factors, &leading_letter_indices)
}

fn get_letter_factors(lhs: &[&str], rhs: &str) -> HashMap<char, i64> {
    let mut letter_compress = HashMap::new();
    for word in lhs {
        for (i, c) in word.chars().rev().enumerate() {
            *letter_compress.entry(c).or_insert(0) += 10_i64.pow(i as u32);
        }
    }

    for (i, c) in rhs.chars().rev().enumerate() {
        *letter_compress.entry(c).or_insert(0) -= 10_i64.pow(i as u32);
    }

    letter_compress
}

fn get_leading_letter_indices(lhs: &[&str], rhs: &str, letters: &[char]) -> Vec<usize> {
    let leading_letters: HashSet<char> = lhs
        .iter()
        .chain(once(&rhs))
        .map(|word| word.chars().next().unwrap())
        .collect();

    letters
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            if leading_letters.contains(l) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn try_solve(
    letters: &[char],
    letter_factors: &[i64],
    leading_letter_indices: &[usize],
) -> Option<HashMap<char, u8>> {
    for p in (0..=9).permutations(letters.len()) {
        // Skip invalid solutions - leading letters cannot be zero
        if leading_letter_indices.iter().any(|&i| p[i] == 0) {
            continue;
        }

        if letter_factors
            .iter()
            .enumerate()
            .map(|(i, f)| f * p[i])
            .sum::<i64>()
            == 0
        {
            return Some(letters.iter().zip(p).map(|(l, v)| (*l, v as u8)).collect());
        }
    }
    None
}
