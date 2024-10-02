use crate::read_values_as;
use crate::utils::io::read_line;
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2037(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (p, w) = read_values_as!(read_line(reader), usize, usize);
    let s = read_line(reader);
    let button_mappings: HashMap<char, (usize, usize)> = [
        vec![' '],
        vec!['A', 'B', 'C'],
        vec!['D', 'E', 'F'],
        vec!['G', 'H', 'I'],
        vec!['J', 'K', 'L'],
        vec!['M', 'N', 'O'],
        vec!['P', 'Q', 'R', 'S'],
        vec!['T', 'U', 'V'],
        vec!['W', 'X', 'Y', 'Z'],
    ]
    .iter()
    .enumerate()
    .flat_map(|(button_index, chars)| {
        chars
            .iter()
            .enumerate()
            .map(move |(char_index, &c)| (c, (button_index, char_index + 1)))
    })
    .collect();

    let mut total_time = 0;
    let mut prev_key = None;
    for c in s.chars() {
        if let Some(&(curr_key, press_times)) = button_mappings.get(&c) {
            if Some(curr_key) == prev_key && curr_key != 0 {
                total_time += w;
            }
            total_time += press_times * p;
            prev_key = Some(curr_key);
        } else {
            eprintln!("Unknown character: {}", c);
        }
    }

    write!(writer, "{}", total_time).expect("Failed to write");
}

// https://www.acmicpc.net/problem/2037
// noinspection SpellCheckingInspection
// 문자메시지
#[test]
fn test_solve2037() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 5
AB"
            .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "1 5
A     A"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "2 10
BB"
            .to_string(),
            want: "18".to_string(),
        },
        TestData {
            s: "2 10
CC"
            .to_string(),
            want: "22".to_string(),
        },
        TestData {
            s: "2 10
ABBAS SALAM"
                .to_string(),
            want: "72".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2037(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
