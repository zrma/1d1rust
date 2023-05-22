use crate::utils::io::read_line;
use std::collections::BTreeMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20920(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect();

    let n = nums[0];
    let m = nums[1];

    let counts = count_words(reader, n, m);
    let sorted_words = sort_words(counts);

    let mut output = String::new();
    for (word, _) in sorted_words {
        output += &format!("{}\n", word);
    }
    writer.write_all(output.as_bytes()).unwrap();
}

fn count_words(reader: &mut impl BufRead, n: i32, m: i32) -> BTreeMap<String, i32> {
    let mut counts = BTreeMap::new();

    for _ in 0..n {
        let word = read_line(reader);
        if word.len() < m as usize {
            continue;
        }

        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

fn sort_words(counts: BTreeMap<String, i32>) -> Vec<(String, i32)> {
    let mut words: Vec<_> = counts.into_iter().collect();

    words.sort_unstable_by(|(a_word, a_count), (b_word, b_count)| {
        let count_cmp = b_count.cmp(a_count);
        let len_cmp = b_word.len().cmp(&a_word.len());
        let word_cmp = a_word.cmp(b_word);

        count_cmp.then(len_cmp).then(word_cmp)
    });

    words
}

// https://www.acmicpc.net/problem/20920
// 영단어 암기는 괴로워
#[test]
fn test_20920() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "7 4
apple
ant
sand
apple
append
sand
sand"
                .to_string(),
            want: "sand
apple
append
"
            .to_string(),
        },
        TestData {
            s: "12 5
appearance
append
attendance
swim
swift
swift
swift
mouse
wallet
mouse
ice
age"
            .to_string(),
            want: "swift
mouse
appearance
attendance
append
wallet
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve20920(&mut reader, &mut writer);

        let res = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(res, data.want, "case: {}", i);
    }
}
