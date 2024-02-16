use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14467(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_observations = read_value(read_line(reader));
    let mut cow_positions = HashMap::new();
    let mut crossings = 0;

    for _ in 0..num_observations {
        let (cow_id, position) = read_values_as!(read_line(reader), String, usize);
        if let Some(&previous_position) = cow_positions.get(&cow_id) {
            if previous_position != position {
                crossings += 1;
            }
        }
        cow_positions.insert(cow_id, position);
    }
    write!(writer, "{}", crossings).expect("Failed to write to output");
}

// https://www.acmicpc.net/problem/14467
// 소가 길을 건너간 이유 1
#[test]
fn test_solve14467() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8
3 1
3 0
6 0
2 1
4 1
3 0
4 0
3 1"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
1 1
1 0
1 1
1 0
1 1"
            .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14467(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
