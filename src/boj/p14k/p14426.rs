use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14426(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let words = read_n_lines(reader, n);
    let trie = build_trie(&words);

    let ans = (0..m)
        .map(|_| read_line(reader))
        .filter(|word| is_prefix(word, &trie))
        .count();

    write!(writer, "{}", ans).expect("write! should work");
}

type Trie = Vec<Vec<usize>>;
const ALPHABET_SIZE: usize = 26;

fn is_prefix(word: &str, trie: &Trie) -> bool {
    word.chars()
        .try_fold(0, |node, c| {
            let index = (c as usize) - ('a' as usize);
            trie.get(node)
                .and_then(|edges| edges.get(index))
                .copied()
                .filter(|&next| next != 0)
        })
        .is_some()
}

fn build_trie(words: &[String]) -> Trie {
    let mut trie = Trie::new();
    trie.push(vec![0; ALPHABET_SIZE]);

    for word in words {
        let mut node = 0;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            if trie[node][index] == 0 {
                trie[node][index] = trie.len();
                trie.push(vec![0; ALPHABET_SIZE]);
            }
            node = trie[node][index];
        }
    }

    trie
}

fn read_n_lines(reader: &mut impl BufRead, n: usize) -> Vec<String> {
    (0..n).map(|_| read_line(reader)).collect()
}

// https://www.acmicpc.net/problem/14426
// noinspection SpellCheckingInspection
// 접두사 찾기
#[test]
fn test_solve14426() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 10
baekjoononlinejudge
startlink
codeplus
sundaycoding
codingsh
baekjoon
star
start
code
sunday
coding
cod
online
judge
plus"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "2 2
hello
world
hell
worl"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 2
hello
world
hell
worr"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14426(&mut reader, &mut writer);

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
