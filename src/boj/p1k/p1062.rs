use crate::read_values_as;
use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1062(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = read_values_as!(read_line(reader), usize, usize);

    if k < 5 {
        write!(writer, "0").expect("Failed to write");
        return;
    } else if k == 26 {
        write!(writer, "{}", n).expect("Failed to write");
        return;
    }

    let essential_letters = ['a', 'c', 'i', 'n', 't'];
    let essential_bitmask = essential_letters
        .iter()
        .fold(0u32, |acc, &c| acc | (1 << char_to_index::<u8>(c)));

    let mut words_bitmasks = Vec::new();
    let mut unique_chars = HashSet::new();

    for _ in 0..n {
        let line = read_line(reader);
        if line.len() < 8 {
            continue;
        }
        let word = &line[4..line.len() - 4]; // Remove "anta" and "tica"
        let mut bitmask = 0u32;
        for c in word.chars() {
            let idx: u32 = char_to_index(c);
            if !essential_letters.contains(&c) {
                bitmask |= 1 << idx;
                unique_chars.insert(idx);
            }
        }
        words_bitmasks.push(bitmask);
    }

    let unique_letters = unique_chars.into_iter().collect::<Vec<_>>();
    let max_teach = k;

    let mut max_readable = 0;

    dfs(
        &unique_letters,
        &words_bitmasks,
        0,
        essential_bitmask,
        max_teach,
        &mut max_readable,
    );

    write!(writer, "{}", max_readable).expect("Failed to write");
}

fn dfs(
    letters: &[u32],
    words_bitmasks: &[u32],
    index: usize,
    selected: u32,
    max_teach: usize,
    max_readable: &mut usize,
) {
    if selected.count_ones().try_into().unwrap_or(usize::MAX) > max_teach {
        return;
    }

    if index == letters.len() {
        let readable = words_bitmasks
            .iter()
            .filter(|&&word| word & !selected == 0)
            .count();
        *max_readable = (*max_readable).max(readable);
        return;
    }

    // Include current letter
    dfs(
        letters,
        words_bitmasks,
        index + 1,
        selected | (1 << letters[index]),
        max_teach,
        max_readable,
    );

    // Exclude current letter
    dfs(
        letters,
        words_bitmasks,
        index + 1,
        selected,
        max_teach,
        max_readable,
    );
}

// https://www.acmicpc.net/problem/2961
// noinspection SpellCheckingInspection
// 가르침
#[test]
fn test_solve1062() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 6
antarctica
antahellotica
antacartica"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 3
antaxxxxxxxtica
antarctica"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "9 8
antabtica
antaxtica
antadtica
antaetica
antaftica
antagtica
antahtica
antajtica
antaktica"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve1062(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }
    }
}
