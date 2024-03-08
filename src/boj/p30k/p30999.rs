use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30999(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_problems, num_voters) = read_values_as!(read_line(reader), usize, usize);

    let num_approved_problems = (0..num_problems)
        .map(|_| {
            read_line(reader)
                .chars()
                .take(num_voters)
                .filter(|&c| c == 'O')
                .count()
                > num_voters / 2
        })
        .filter(|&is_approved| is_approved)
        .count();

    write!(writer, "{}", num_approved_problems).expect("Failed to write");
}

// https://www.acmicpc.net/problem/30999
// noinspection SpellCheckingInspection
// 민주주의
#[test]
fn test_solve30999() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 3
OOX
OXX"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 3
OOX
OOX
OOX"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3 3
OXO
XXO
OOO"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve30999(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
