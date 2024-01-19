use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12933(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut quack = Vec::new();

    for c in s.chars() {
        if !process_char(&mut quack, c) {
            write!(writer, "-1").unwrap();
            return;
        }
    }

    if is_quack_sequence_incomplete(&quack) {
        write!(writer, "-1").unwrap();
    } else {
        write!(writer, "{}", quack.len()).unwrap();
    }
}

fn process_char(quack: &mut Vec<Vec<i32>>, c: char) -> bool {
    let curr = char_to_index(c);

    for q in quack.iter_mut() {
        if q.last() == Some(&((curr + 4) % 5)) {
            q.push(curr);
            return true;
        }
    }

    if curr == 0 {
        quack.push(vec![0]);
        return true;
    }

    false
}

fn is_quack_sequence_incomplete(quack: &[Vec<i32>]) -> bool {
    quack.iter().any(|q| *q.last().unwrap() != 4)
}

fn char_to_index(c: char) -> i32 {
    match c {
        'q' => 0,
        'u' => 1,
        'a' => 2,
        'c' => 3,
        _ => 4,
    }
}

// https://www.acmicpc.net/problem/12933
// noinspection SpellCheckingInspection
// 오리
#[test]
fn test1_12933() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "quqacukqauackck".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "kcauq".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "quackquackquackquackquackquackquackquackquackquack".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "qqqqqqqqqquuuuuuuuuuaaaaaaaaaacccccccccckkkkkkkkkk".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "quqaquuacakcqckkuaquckqauckack".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "quackqauckquack".to_string(),
            want: "-1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12933(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
