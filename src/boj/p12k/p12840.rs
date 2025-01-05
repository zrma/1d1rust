use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12840(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (h, m, s) = read_values_as!(read_line(reader), u32, u32, u32);
    let mut seconds = to_seconds(h, m, s);
    let num_queries: usize = read_value(read_line(reader));

    for _ in 0..num_queries {
        let input = read_line(reader);
        if input == "3" {
            writeln!(writer, "{}", to_timestamp(seconds)).unwrap();
        } else {
            let (t, x) = read_values_as!(input, u32, u32);
            seconds = adjust_time(seconds, t, x);
        }
    }
}

fn to_seconds(h: u32, m: u32, s: u32) -> u32 {
    h * 3600 + m * 60 + s
}

fn to_timestamp(seconds: u32) -> String {
    format!(
        "{} {} {}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

fn adjust_time(seconds: u32, t: u32, x: u32) -> u32 {
    const DAY_SECONDS: u32 = 24 * 3600;

    let adjusted_x = x % DAY_SECONDS;
    match t {
        1 => (seconds + adjusted_x) % DAY_SECONDS,
        2 => (DAY_SECONDS + seconds - adjusted_x) % DAY_SECONDS,
        _ => unreachable!(),
    }
}

// https://www.acmicpc.net/problem/12840
// 창용이의 시계
#[test]
fn test_solve12840() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 1 0
4
1 4263
3
2 1175
3"
            .to_string(),
            want: "1 12 3
0 52 28"
                .to_string(),
        },
        TestData {
            s: "0 0 0
4
1 0
2 0
3
3"
            .to_string(),
            want: "0 0 0
0 0 0"
                .to_string(),
        },
        TestData {
            s: "0 0 0
2
2 86400
3"
            .to_string(),
            want: "0 0 0".to_string(),
        },
        TestData {
            s: "0 0 0
2
2 864000
3"
            .to_string(),
            want: "0 0 0".to_string(),
        },
        TestData {
            s: "23 59 59
2
1 1
3"
            .to_string(),
            want: "0 0 0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve12840(&mut reader, &mut writer);

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
