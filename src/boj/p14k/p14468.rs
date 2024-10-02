use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14468(reader: &mut impl BufRead, writer: &mut impl Write) {
    let sequence = read_line(reader);

    #[derive(Copy, Clone, Default)]
    struct Position {
        start: usize,
        end: usize,
    }

    let positions = sequence.chars().enumerate().fold(
        [Position { start: 0, end: 0 }; 26],
        |mut acc, (index, char)| {
            let alphabet_index = char as usize - 'A' as usize;
            if acc[alphabet_index].start == 0 {
                acc[alphabet_index].start = index + 1;
            } else {
                acc[alphabet_index].end = index + 1;
            }
            acc
        },
    );

    // ┌─────┐
    // │  ┌──│──┐
    // │  │  │  │
    // A  B  A  B
    let cross = |a: &Position, b: &Position| a.start < b.start && b.start < a.end && a.end < b.end;

    let mut crossing_count = 0;
    for i in 0..26 {
        for j in 0..26 {
            let a = &positions[i];
            let b = &positions[j];
            if i != j && cross(a, b) {
                crossing_count += 1;
            }
        }
    }

    write!(writer, "{}", crossing_count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14468
// noinspection SpellCheckingInspection
// 소가 길을 건너간 이유 2
#[test]
fn test_solve14468() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ABCCABDDEEFFGGHHIIJJKKLLMMNNOOPPQQRRSSTTUUVVWWXXYYZZ".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "AABBCCDDEEFFGGHHIIJJKKLLMMNNOOPPQQRRSSTTUUVVWWXXYYZZ".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            want: "325".to_string(),
        },
        TestData {
            s: "ABCDEFGHIJKLMNOPQRSTUVWXYZAZYXWVUTSRQPONMLKJIHGFEDCB".to_string(),
            want: "25".to_string(),
        },
        TestData {
            s: "ZYXWVUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "AZYXWVUTSRQPONMLKJIHGFEDCBABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            want: "25".to_string(),
        },
        TestData {
            s: "AACBCB".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "AABCBC".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14468(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
