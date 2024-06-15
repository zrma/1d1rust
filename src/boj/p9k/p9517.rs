use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9517(reader: &mut impl BufRead, writer: &mut impl Write) {
    const MAX_TIME: i32 = 210;
    const MAX_PLAYER: i32 = 8;

    let k: i32 = read_value(read_line(reader));
    let n: i32 = read_value(read_line(reader));

    let mut time = 0;
    let mut player = k;

    for _ in 0..n {
        let (t, z) = read_values_as!(read_line(reader), i32, char);
        time += t;
        if time >= MAX_TIME {
            break;
        }

        if z == 'T' {
            player = if player == MAX_PLAYER { 1 } else { player + 1 };
        }
    }

    write!(writer, "{}", player).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9517
// 아이 러브 크로아티아
#[test]
fn test_solve9517() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
5
20 T
50 T
80 T
50 T
30 T"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "1
12
20 T
20 T
20 T
20 T
20 T
20 T
20 T
20 T
20 T
20 T
20 T
20 T"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3
5
100 T
100 N
100 T
100 T
100 N"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5
6
70 T
50 P
30 N
50 T
30 P
80 T"
                .to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve9517(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
