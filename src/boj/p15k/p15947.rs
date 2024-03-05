use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15947(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let lyrics = vec![
        "baby", "sukhwan", "tururu", "turu", "very", "cute", "tururu", "turu", "in", "bed",
        "tururu", "turu", "baby", "sukhwan",
    ];

    let mod_n = (n - 1) % 14;
    let div_n = (n - 1) / 14;

    let ans = match mod_n {
        2 | 6 | 10 => {
            if div_n >= 3 {
                format!("tu+ru*{}", div_n + 2)
            } else {
                format!("tururu{}", "ru".repeat(div_n))
            }
        }
        3 | 7 | 11 => {
            if div_n >= 4 {
                format!("tu+ru*{}", div_n + 1)
            } else {
                format!("turu{}", "ru".repeat(div_n))
            }
        }
        _ => lyrics[mod_n].to_string(),
    };

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/15947
// 아기 석환 뚜루루 뚜루
// noinspection SpellCheckingInspection
#[test]
fn test_solve15947() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "baby".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "sukhwan".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "tururu".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "turu".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "very".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "cute".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "tururu".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "turu".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "in".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "bed".to_string(),
        },
        TestData {
            s: "11".to_string(),
            want: "tururu".to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "turu".to_string(),
        },
        TestData {
            s: "13".to_string(),
            want: "baby".to_string(),
        },
        TestData {
            s: "14".to_string(),
            want: "sukhwan".to_string(),
        },
        TestData {
            s: "17".to_string(),
            want: "turururu".to_string(),
        },
        TestData {
            s: "18".to_string(),
            want: "tururu".to_string(),
        },
        TestData {
            s: "45".to_string(),
            want: "tu+ru*5".to_string(),
        },
        TestData {
            s: "46".to_string(),
            want: "tururururu".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15947(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
