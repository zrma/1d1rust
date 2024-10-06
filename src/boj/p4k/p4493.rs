use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4493(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let result = (0..num_cases)
        .map(|_| play_game(reader))
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", result).unwrap();
}

fn play_game(reader: &mut impl BufRead) -> String {
    let num_rounds: usize = read_value(read_line(reader));

    let (p1_wins, p2_wins) = (0..num_rounds)
        .map(|_| read_values_as!(read_line(reader), char, char))
        .fold((0, 0), |(p1_wins, p2_wins), (p1, p2)| match (p1, p2) {
            ('R', 'S') | ('S', 'P') | ('P', 'R') => (p1_wins + 1, p2_wins),
            ('S', 'R') | ('P', 'S') | ('R', 'P') => (p1_wins, p2_wins + 1),
            _ => (p1_wins, p2_wins),
        });

    match p1_wins.cmp(&p2_wins) {
        Greater => "Player 1".to_string(),
        Less => "Player 2".to_string(),
        Equal => "TIE".to_string(),
    }
}

// https://www.acmicpc.net/problem/4493
// 가위 바위 보?
#[test]
fn test_solve4493() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2
R P
S R
3
P P
R S
S R
1
P R"
            .to_string(),
            want: "Player 2
TIE
Player 1"
                .to_string(),
        },
        TestData {
            s: "1
1
P R"
            .to_string(),
            want: "Player 1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4493(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
