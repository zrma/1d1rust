use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10984(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases: usize = read_value(read_line(reader));
    let ans = (0..num_of_cases)
        .map(|_| {
            let total_subjects: usize = read_value(read_line(reader));
            let (mut total_credits, mut weighted_sum) = (0f32, 0f32);
            for _ in 0..total_subjects {
                let (credits, grade) = read_values_as!(read_line(reader), f32, f32);
                total_credits += credits;
                weighted_sum += credits * grade;
            }
            let avg_grade = weighted_sum / total_credits;
            format!("{} {:.1}", total_credits, avg_grade)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/10984
// 내 학점을 구해줘
#[test]
fn test_solve10984() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
4
3 4.3
2 2.0
4 0.0
2 4.0
3
4 0.0
4 0.0
3 0.0"
                .to_string(),
            want: "11 2.3
11 0.0"
                .to_string(),
        },
        TestData {
            s: "2
4
3 4.3
2 2
4 0.0
2 4.0
3
4 0.0
4 0
3 0"
            .to_string(),
            want: "11 2.3
11 0.0"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10984(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
