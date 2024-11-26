use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20529(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_line(reader).parse().unwrap();
    for _ in 0..t {
        let n: usize = read_line(reader).parse().unwrap();

        let s = read_line(reader);

        if n > 32 {
            writeln!(writer, "0").expect("Failed to write");
            continue;
        }

        let values = s.split_whitespace().take(n).collect::<Vec<_>>();

        let mut min_dist = usize::MAX;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let dist = calc_dist(values[i], values[j])
                        + calc_dist(values[j], values[k])
                        + calc_dist(values[k], values[i]);
                    min_dist = min_dist.min(dist);
                }
            }
        }

        writeln!(writer, "{}", min_dist).expect("Failed to write");
    }
}

fn calc_dist(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

// https://www.acmicpc.net/problem/20529
// 가장 가까운 세 사람의 심리적 거리
#[test]
fn test_solve20529() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
3
ENTJ INTP ESFJ
4
ESFP ESFP ESFP ESFP
5
INFP INFP ESTP ESTJ ISTJ"
                .to_string(),
            want: "8
0
4
"
            .to_string(),
        },
        TestData {
            s: "1
5
INFP INFP ESTP ESTJ ISTJ"
                .to_string(),
            want: "4
"
            .to_string(),
        },
        TestData {
            s: "1
33
ISTJ ISFJ INFJ INTJ ISTP ISFP INFP INTP ESTP ESFP ENFP ENTP ESTJ ESFJ ENFJ ENTJ ISTJ ISFJ INFJ INTJ ISTP ISFP INFP INTP ESTP ESFP ENFP ENTP ESTJ ESFJ ENFJ ENTJ ISTJ"
            .to_string(),
            want: "0
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20529(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got.trim(), data.want.trim(), "failed at {} with {}", i, data.s);
    }
}
