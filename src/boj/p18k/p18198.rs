use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18198(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let s = s.as_bytes();

    let (score_a, score_b) = s.windows(2).fold((0, 0), |(a, b), pair| match pair {
        b"A1" => (a + 1, b),
        b"A2" => (a + 2, b),
        b"B1" => (a, b + 1),
        b"B2" => (a, b + 2),
        _ => (a, b),
    });

    let winner = if score_a > score_b { "A" } else { "B" };
    writeln!(writer, "{}", winner).unwrap();
}

#[allow(dead_code)]
fn solve18198_iter(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let s = s.as_bytes();

    let mut score_a = 0;
    let mut score_b = 0;

    for pair in s.windows(2) {
        match pair {
            b"A1" => score_a += 1,
            b"A2" => score_a += 2,
            b"B1" => score_b += 1,
            b"B2" => score_b += 2,
            _ => {}
        }
    }

    let winner = if score_a > score_b { "A" } else { "B" };
    writeln!(writer, "{}", winner).unwrap();
}

// https://www.acmicpc.net/problem/18198
// noinspection SpellCheckingInspection
// Basketball One-on-One
#[test]
fn test_solve18198() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "A2B1A2B2A1A2A2A2".to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "A2B2A1B2A2B1A2B2A1B2A1A1B1A1A2".to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "B2A1B2A2B1B2B2B2".to_string(),
            want: "B".to_string(),
        },
        TestData {
            s: "B2A2B1A2B2A1B2A2B1A2B1B1A1B1B2".to_string(),
            want: "B".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve18198(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve18198_iter(&mut reader, &mut writer);

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
}
