use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1380(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut cnt = 0;

    loop {
        let n: usize = read_value(read_line(reader));
        if n == 0 {
            break;
        }
        cnt += 1;

        let names = (0..n).map(|_| read_line(reader)).collect::<Vec<_>>();
        let numbers = (0..2 * n - 1)
            .map(|_| {
                let line = read_line(reader);
                read_values_as!(line, usize, String).0
            })
            .collect::<Vec<_>>();

        let unique_number = numbers.iter().fold(0, |acc, &i| acc ^ i);
        let lost_name = &names[unique_number - 1];

        writeln!(writer, "{} {}", cnt, lost_name).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/1380
// noinspection SpellCheckingInspection
// 귀걸이
#[test]
fn test_solve1380() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
Betty Boolean
Alison Addaway
Carrie Carryon
1 B
2 A
3 B
3 A
1 A
2
Helen Clark
Margaret Thatcher
1 B
2 B
2 A
0"
            .to_string(),
            want: "1 Alison Addaway
2 Helen Clark
"
            .to_string(),
        },
        TestData {
            s: "2
Helen Clark
Margaret Thatcher
1 B
2 B
2 A
0"
            .to_string(),
            want: "1 Helen Clark
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1380(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
