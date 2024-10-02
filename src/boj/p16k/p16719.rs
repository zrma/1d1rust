use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16719(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let sequence = s.as_bytes();

    let mut selected_flags = vec![false; sequence.len()];
    let mut subsequences = Vec::new();

    collect_subsequences(
        sequence,
        &mut selected_flags,
        0,
        sequence.len() - 1,
        &mut subsequences,
    );

    write!(writer, "{}", subsequences.join("\n")).unwrap();
}

fn collect_subsequences(
    sequence: &[u8],
    selected_flags: &mut Vec<bool>,
    start: usize,
    end: usize,
    subsequences: &mut Vec<String>,
) {
    if start > end || start >= sequence.len() || end >= sequence.len() {
        return;
    }

    if let Some(idx) = find_min_unselected(sequence, selected_flags, start, end) {
        selected_flags[idx] = true;
        subsequences.push(construct_subsequence(sequence, selected_flags));

        collect_subsequences(sequence, selected_flags, idx + 1, end, subsequences);
        collect_subsequences(
            sequence,
            selected_flags,
            start,
            idx.saturating_sub(1),
            subsequences,
        );
    }
}

fn find_min_unselected(
    sequence: &[u8],
    selected_flags: &[bool],
    start: usize,
    end: usize,
) -> Option<usize> {
    sequence
        .iter()
        .enumerate()
        .filter(|(i, _)| *i >= start && *i <= end && !selected_flags[*i])
        .min_by_key(|&(_, &val)| val)
        .map(|(i, _)| i)
}

fn construct_subsequence(sequence: &[u8], selected_flags: &[bool]) -> String {
    sequence
        .iter()
        .enumerate()
        .filter(|(i, _)| selected_flags[*i])
        .map(|(_, &val)| val as char)
        .collect()
}

// https://www.acmicpc.net/problem/16719
// noinspection SpellCheckingInspection
// ZOAC
#[test]
fn test_solve16719() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ZOAC".to_string(),
            want: "A
AC
OAC
ZOAC"
                .to_string(),
        },
        TestData {
            s: "BAC".to_string(),
            want: "A
AC
BAC"
            .to_string(),
        },
        TestData {
            s: "A".to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "STARTLINK".to_string(),
            want: "A
AI
AIK
AINK
ALINK
ARLINK
ARTLINK
SARTLINK
STARTLINK"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16719(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
