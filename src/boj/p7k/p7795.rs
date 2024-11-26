use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7795(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let mut ans = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let (n, m) = read_values_as!(read_line(reader), usize, usize);

        let mut a: Vec<i32> = read_n_values(reader, n);
        let mut b: Vec<i32> = read_n_values(reader, m);

        a.sort_unstable();
        b.sort_unstable();

        let mut count = 0;
        let mut j = 0;
        for &ai in &a {
            while j < m && b[j] < ai {
                j += 1;
            }
            count += j;
        }

        ans.push(count);
    }

    let output = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    write!(writer, "{}", output).unwrap();
}

// https://www.acmicpc.net/problem/7795
// 먹을 것인가 먹힐 것인가
#[test]
fn test_solve7795() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
5 3
8 1 7 3 1
3 6 1
3 4
2 13 7
103 11 290 215"
                .to_string(),
            want: "7
1"
            .to_string(),
        },
        TestData {
            s: "1
1 1
1
1"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1
1 1
2
1"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7795(&mut reader, &mut writer);

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
