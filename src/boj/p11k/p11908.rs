use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11908(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cards: usize = read_value(read_line(reader));
    let cards: Vec<usize> = read_n_values(reader, num_cards);

    let max_card = cards.iter().max().copied().unwrap_or(0);
    let total_sum: usize = cards.iter().sum();

    writeln!(writer, "{}", total_sum - max_card).unwrap();
}

// https://www.acmicpc.net/problem/11908
// 카드
#[test]
fn test_solve11908() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "2
3 4"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3
1 3 5"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "20
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20"
                .to_string(),
            want: "190".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11908(&mut reader, &mut writer);

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
