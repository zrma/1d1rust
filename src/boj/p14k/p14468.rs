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

    let mut positions = [Position { start: 0, end: 0 }; 26];
    for (index, ch) in sequence.chars().enumerate() {
        let alphabet_index = ch as usize - 'A' as usize;
        let pos = &mut positions[alphabet_index];
        if pos.start == 0 {
            pos.start = index + 1;
        } else {
            pos.end = index + 1;
        }
    }

    // ┌─────┐
    // │  ┌──│──┐
    // │  │  │  │
    // A  B  A  B
    let cross = |a: &Position, b: &Position| a.start < b.start && b.start < a.end && a.end < b.end;

    let mut crossing_count = 0;
    for i in 0..26 {
        for j in 0..26 {
            if i != j && cross(&positions[i], &positions[j]) {
                crossing_count += 1;
            }
        }
    }

    writeln!(writer, "{}", crossing_count).unwrap();
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
