use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2309(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut heights: Vec<i32> = (0..9).map(|_| read_value(read_line(reader))).collect();
    heights.sort_unstable();

    let total_sum = heights.iter().sum();
    let (height_to_remove1, height_to_remove2) = find_two_heights_to_remove(&heights, total_sum);

    heights.retain(|&h| h != height_to_remove1 && h != height_to_remove2);

    for height in heights {
        writeln!(writer, "{}", height).expect("writeln! should work");
    }
}

fn find_two_heights_to_remove(heights: &[i32], total_sum: i32) -> (i32, i32) {
    heights
        .iter()
        .enumerate()
        .find_map(|(i, &h1)| {
            heights.iter().skip(i + 1).find_map(|&h2| {
                if total_sum - h1 - h2 == 100 {
                    Some((h1, h2))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

// https://www.acmicpc.net/problem/2309
// 일곱 난쟁이
#[test]
fn test_solve2309() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "20
7
23
19
10
15
25
8
13"
            .to_string(),
            want: "7
8
10
13
19
20
23
"
            .to_string(),
        },
        TestData {
            s: "7
8
10
13
19
20
23
25
15"
            .to_string(),
            want: "7
8
10
13
19
20
23
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2309(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
