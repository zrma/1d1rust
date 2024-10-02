use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6443(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let mut all_permutations = Vec::new();

    for _ in 0..num_cases {
        let mut characters = read_line(reader).chars().collect::<Vec<_>>();
        characters.sort_unstable();
        let mut used_flags = vec![false; characters.len()];
        let mut current_permutation = Vec::new();
        generate_permutations(
            &characters,
            &mut used_flags,
            &mut current_permutation,
            &mut all_permutations,
        );
    }

    write!(writer, "{}", all_permutations.join("\n")).unwrap();
}

fn generate_permutations(
    characters: &[char],
    used_flags: &mut Vec<bool>,
    current_permutation: &mut Vec<char>,
    all_permutations: &mut Vec<String>,
) {
    if current_permutation.len() == characters.len() {
        all_permutations.push(current_permutation.iter().collect());
        return;
    }

    for (i, &ch) in characters.iter().enumerate() {
        if used_flags[i] || should_skip(characters, used_flags, i) {
            continue;
        }
        used_flags[i] = true;
        current_permutation.push(ch);
        generate_permutations(
            characters,
            used_flags,
            current_permutation,
            all_permutations,
        );
        current_permutation.pop();
        used_flags[i] = false;
    }
}

fn should_skip(characters: &[char], used_flags: &[bool], index: usize) -> bool {
    index > 0 && characters[index] == characters[index - 1] && !used_flags[index - 1]
}

// https://www.acmicpc.net/problem/6443
// noinspection SpellCheckingInspection
// 애너그램
#[test]
fn test_solve6443() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
abc
acba"
                .to_string(),
            want: "abc
acb
bac
bca
cab
cba
aabc
aacb
abac
abca
acab
acba
baac
baca
bcaa
caab
caba
cbaa"
                .to_string(),
        },
        TestData {
            s: "1
abcd"
                .to_string(),
            want: "abcd
abdc
acbd
acdb
adbc
adcb
bacd
badc
bcad
bcda
bdac
bdca
cabd
cadb
cbad
cbda
cdab
cdba
dabc
dacb
dbac
dbca
dcab
dcba"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6443(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
