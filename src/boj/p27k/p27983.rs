use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27983(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_ribbons: usize = read_value(read_line(reader));

    let positions: Vec<i64> = read_n_values(reader, num_ribbons);
    let lengths: Vec<u64> = read_n_values(reader, num_ribbons);
    let colors: Vec<char> = read_n_values(reader, num_ribbons);

    let mut found_pair = None;

    'outer: for i in 0..num_ribbons {
        for j in i + 1..num_ribbons {
            if colors[i] != colors[j]
                && positions[i].abs_diff(positions[j]) <= lengths[i] + lengths[j]
            {
                found_pair = Some((i + 1, j + 1));
                break 'outer;
            }
        }
    }

    if let Some((first, second)) = found_pair {
        writeln!(writer, "YES\n{} {}", first, second).unwrap();
    } else {
        writeln!(writer, "NO").unwrap();
    }
}

// https://www.acmicpc.net/problem/27983
// 리본 (Easy)
#[test]
fn test_solve27983() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1 3
1 1
Y R"
            .to_string(),
            want: "YES
1 2"
            .to_string(),
        },
        TestData {
            s: "2
-4 1
1 2
B B"
            .to_string(),
            want: "NO".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27983(&mut reader, &mut writer);

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
