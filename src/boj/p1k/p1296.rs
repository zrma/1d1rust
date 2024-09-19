use crate::utils::io::read_line;
use std::cmp::Reverse;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1296(reader: &mut impl BufRead, writer: &mut impl Write) {
    let name = read_line(reader);
    let (l, o, v, e) = initial_count(&name);

    let n: usize = read_line(reader).parse().unwrap();
    let ans = (0..n)
        .map(|_| read_line(reader))
        .max_by_key(|name| {
            let (l, o, v, e) = update_count(name, l, o, v, e);
            (calc(l, o, v, e), Reverse(name.clone()))
        })
        .unwrap();

    write!(writer, "{}", ans).expect("Failed to write");
}

fn initial_count(s: &str) -> (usize, usize, usize, usize) {
    (count(s, 'L'), count(s, 'O'), count(s, 'V'), count(s, 'E'))
}

fn update_count(s: &str, l: usize, o: usize, v: usize, e: usize) -> (usize, usize, usize, usize) {
    (
        count(s, 'L') + l,
        count(s, 'O') + o,
        count(s, 'V') + v,
        count(s, 'E') + e,
    )
}

fn count(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}

fn calc(l: usize, o: usize, v: usize, e: usize) -> usize {
    ((l + o) * (l + v) * (l + e) * (o + v) * (o + e) * (v + e)) % 100
}

// https://www.acmicpc.net/problem/1296
// 팀 이름 정하기
// noinspection SpellCheckingInspection
#[test]
fn test_solve1296() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "LOVE
3
JACOB
FRANK
DANO"
                .to_string(),
            want: "FRANK".to_string(),
        },
        TestData {
            s: "JANE
4
THOMAS
MICHAEL
INDY
LIU"
            .to_string(),
            want: "INDY".to_string(),
        },
        TestData {
            s: "LILLY
1
PIERRE"
                .to_string(),
            want: "PIERRE".to_string(),
        },
        TestData {
            s: "MERYLOV
5
JOHN
DAVE
STEVE
JOHN
DAVE"
                .to_string(),
            want: "DAVE".to_string(),
        },
        TestData {
            s: "LLOL
4
BVERON
CVERON
AVERON
DVERON"
                .to_string(),
            want: "AVERON".to_string(),
        },
        TestData {
            s: "VELYLEOCEVE
5
YVXHOVE
LCOKO
OGWSJVEVEDLE
WGFVSJEL
VLOLUVCBLLQVESWHEEKC"
                .to_string(),
            want: "VLOLUVCBLLQVESWHEEKC".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1296(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
