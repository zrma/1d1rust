use crate::utils::io::{read_line, read_n_values, read_value};
use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1267(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_calls = read_value::<usize>(read_line(reader));
    let call_durations = read_n_values::<usize>(reader, num_calls);

    let (y_cost, m_cost) = call_durations.iter().fold((0, 0), |(y, m), &dur| {
        let y_add = (dur / 30 + 1) * 10;
        let m_add = (dur / 60 + 1) * 15;
        (y + y_add, m + m_add)
    });

    let ans = match y_cost.cmp(&m_cost) {
        Ordering::Less => format!("Y {}", y_cost),
        Ordering::Greater => format!("M {}", m_cost),
        Ordering::Equal => format!("Y M {}", y_cost),
    };

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1267
// noinspection SpellCheckingInspection
// 핸드폰 요금
#[test]
fn test_solve1267() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
40 40 40"
                .to_string(),
            want: "M 45".to_string(),
        },
        TestData {
            s: "3
61 61 61"
                .to_string(),
            want: "Y M 90".to_string(),
        },
        TestData {
            s: "2
61 10"
                .to_string(),
            want: "Y 40".to_string(),
        },
        TestData {
            s: "2
60 65"
                .to_string(),
            want: "Y M 60".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1267(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
