use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3058(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));
    let ans: Vec<String> = (0..t)
        .map(|_| {
            let v = read_n_values(reader, 7).into_iter().filter(|&x| x % 2 == 0);
            let (sum, min) = v.fold((0, i32::MAX), |(sum, min), x| (sum + x, min.min(x)));
            format!("{} {}", sum, min)
        })
        .collect();

    write!(writer, "{}", ans.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/3058
// 짝수를 찾아라
#[test]
fn test_solve3058() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1 2 3 4 5 6 7
13 78 39 42 54 93 86"
                .to_string(),
            want: "12 2
260 42"
                .to_string(),
        },
        TestData {
            s: "1
1 3 5 7 9 11 12"
                .to_string(),
            want: "12 12".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3058(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
