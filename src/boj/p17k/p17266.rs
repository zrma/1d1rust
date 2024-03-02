use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17266(reader: &mut impl BufRead, writer: &mut impl Write) {
    let bridge_length = read_value(read_line(reader));
    let num_lights = read_value(read_line(reader));
    let mut light_positions = read_n_values(reader, num_lights);
    light_positions.sort_unstable();

    let min_height = find_min_height(&light_positions, bridge_length);
    write!(writer, "{}", min_height).unwrap();
}

fn find_min_height(light_positions: &[usize], bridge_length: usize) -> usize {
    let (mut low, mut high) = (0, bridge_length);

    while low < high {
        let mid = (low + high) / 2;
        if is_light_coverage_sufficient(light_positions, mid, bridge_length) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn is_light_coverage_sufficient(
    light_positions: &[usize],
    height: usize,
    bridge_length: usize,
) -> bool {
    let mut last_covered = 0;
    for &position in light_positions {
        if position.abs_diff(last_covered) > height {
            return false;
        }
        last_covered = position + height;
    }
    last_covered >= bridge_length
}

#[test]
// https://www.acmicpc.net/problem/17266
// 어두운 굴다리
fn test_solve17266() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
2
2 4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3
1
0"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "8
2
3 7"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "7
2
0 5"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "4
2
0 4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
2
0 1"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10
2
0 9 "
                .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17266(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
