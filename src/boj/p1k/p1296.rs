use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1296(reader: &mut impl BufRead, writer: &mut impl Write) {
    let name = read_line(reader);
    let l = count(&name, 'L');
    let o = count(&name, 'O');
    let v = count(&name, 'V');
    let e = count(&name, 'E');

    let mut max = 0;
    let mut ans = String::new();

    let n = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..n {
        let name = read_line(reader);
        let l = count(&name, 'L') + l;
        let o = count(&name, 'O') + o;
        let v = count(&name, 'V') + v;
        let e = count(&name, 'E') + e;

        let score = calc(l, o, v, e);
        if score > max {
            max = score;
            ans = name;
        } else if score == max && (ans.is_empty() || name < ans) {
            ans = name;
        }
    }

    write!(writer, "{}", ans).unwrap();
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
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
