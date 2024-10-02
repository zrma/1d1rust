use crate::utils::io::{read_line, read_n_values, read_value};
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{BufRead, Write};
use std::iter::FromIterator;

#[allow(dead_code)]
fn solve2776(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let originals: HashSet<i32> = read_collection(reader);
        let queries: Vec<i32> = read_collection(reader);

        let ans = queries
            .iter()
            .map(|q| if originals.contains(q) { "1" } else { "0" })
            .collect::<Vec<&str>>()
            .join("\n");
        answers.push(ans);
    }

    write!(writer, "{}", answers.join("\n")).unwrap();
}

fn read_collection<T, C>(reader: &mut impl BufRead) -> C
where
    T: std::str::FromStr,
    T: Eq + Hash,
    T::Err: std::fmt::Debug,
    C: FromIterator<T>,
{
    let n: usize = read_value(read_line(reader));
    read_n_values::<T>(reader, n).into_iter().collect()
}

// https://www.acmicpc.net/problem/2776
// 암기왕
#[test]
fn test_solve2776() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1
5
4 1 5 2 3
5
1 3 7 9 5"
                .to_string(),
            want: "1
1
0
0
1"
            .to_string(),
        },
        TestData {
            s: "2
1
10000000
1
1
2
10000000 10000001
2
1 10000000"
                .to_string(),
            want: "0
0
1"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2776(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
