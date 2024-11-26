use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9469(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut res = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let (idx, d, a, b, f) = read_values_as!(read_line(reader), i32, f64, f64, f64, f64);
        let time = d / (a + b) * f;
        res.push(format!("{} {:.6}", idx, time));
    }

    write!(writer, "{}", res.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9469
// 폰 노이만
#[test]
fn test_solve9469() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 250 10 15 20
2 10.7 3.5 4.7 5.5
3 523.7 15.3 20.7 33.3
4 1000 30 30 50
5 500 15 15 25"
                .to_string(),
            want: "1 200.000000
2 7.176829
3 484.422500
4 833.333333
5 416.666667"
                .to_string(),
        },
        TestData {
            s: "1
1 2 1 1 1"
                .to_string(),
            want: "1 1.000000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9469(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
