use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12851(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (start, end) = read_values_as!(read_line(reader), usize, usize);
    let mut way = vec![0; 100001];
    let mut current_level = Vec::new();
    let mut min_time = 0;
    let mut num_ways = 0;

    current_level.push(start);
    while !current_level.is_empty() {
        let mut next_level = Vec::new();
        for &current in &current_level {
            if current == end {
                num_ways += 1;
            }

            for &next in [
                current.checked_sub(1),
                current.checked_add(1),
                current.checked_mul(2),
            ]
            .iter()
            {
                let next = match next {
                    Some(v) => v,
                    None => continue,
                };
                if !(0..=100000).contains(&next) || way[next] != 0 && way[next] != way[current] + 1
                {
                    continue;
                }

                way[next] = way[current] + 1;
                next_level.push(next);
            }
        }

        if num_ways > 0 {
            break;
        }
        min_time += 1;
        current_level = next_level;
    }

    writeln!(writer, "{}", min_time).expect("Failed to write");
    write!(writer, "{}", num_ways).expect("Failed to write");
}

// https://www.acmicpc.net/problem/12851
// 숨바꼭질 2
#[test]
fn test_solve12851() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 17".to_string(),
            want: "4
2"
            .to_string(),
        },
        TestData {
            s: "0 0".to_string(),
            want: "0
1"
            .to_string(),
        },
        TestData {
            s: "4 5".to_string(),
            want: "1
1"
            .to_string(),
        },
        TestData {
            s: "7 0".to_string(),
            want: "7
1"
            .to_string(),
        },
        TestData {
            s: "0 3".to_string(),
            want: "3
2"
            .to_string(),
        },
        TestData {
            s: "4 9".to_string(),
            want: "2
1"
            .to_string(),
        },
        TestData {
            s: "0 100000".to_string(),
            want: "22
8"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12851(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
