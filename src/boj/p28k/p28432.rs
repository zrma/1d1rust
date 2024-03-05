use crate::utils::io::{read_line, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28432(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let words: Vec<String> = (0..n).map(|_| read_line(reader)).collect();

    let m = read_value(read_line(reader));
    let mut nominees: HashSet<String> = (0..m).map(|_| read_line(reader)).collect();

    if n == 1 {
        write!(writer, "{}", nominees.iter().next().unwrap()).unwrap();
        return;
    }

    words.iter().for_each(|word| {
        nominees.remove(word);
    });

    let pos = words.iter().position(|x| x == "?").unwrap();
    let (prev_char, next_char) = find_surrounding_chars(&words, pos);

    if let Some(nominee) = nominees
        .into_iter()
        .find(|nominee| matches_nominee(nominee, prev_char, next_char))
    {
        write!(writer, "{}", nominee).unwrap();
    }
}

fn find_surrounding_chars(words: &[String], pos: usize) -> (Option<char>, Option<char>) {
    let prev_char = pos.checked_sub(1).and_then(|i| words[i].chars().last());
    let next_char = words.get(pos + 1).and_then(|s| s.chars().next());
    (prev_char, next_char)
}

fn matches_nominee(nominee: &str, prev_char: Option<char>, next_char: Option<char>) -> bool {
    match (prev_char, next_char) {
        (Some(p), Some(n)) => nominee.starts_with(p) && nominee.ends_with(n),
        (Some(p), None) => nominee.starts_with(p),
        (None, Some(n)) => nominee.ends_with(n),
        _ => false,
    }
}

// https://www.acmicpc.net/problem/28432
// noinspection SpellCheckingInspection
// 끝말잇기
#[test]
fn test_solve28432() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
charlie
echo
?
romeo
oscar
3
alfa
oscar
or"
            .to_string(),
            want: "or".to_string(),
        },
        TestData {
            s: "3
charlie
?
echo
3
oscar
alfa
eye"
            .to_string(),
            want: "eye".to_string(),
        },
        TestData {
            s: "3
charlie
echo
?
3
oscar
alfa
eye"
            .to_string(),
            want: "oscar".to_string(),
        },
        TestData {
            s: "3
?
echo
oscar
3
oscar
alfa
eye"
            .to_string(),
            want: "eye".to_string(),
        },
        TestData {
            s: "3
?
ras
sar
2
ar
as"
            .to_string(),
            want: "ar".to_string(),
        },
        TestData {
            s: "2
aaaab
?
2
baa
aab"
            .to_string(),
            want: "baa".to_string(),
        },
        TestData {
            s: "1
?
1
abc"
            .to_string(),
            want: "abc".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28432(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
