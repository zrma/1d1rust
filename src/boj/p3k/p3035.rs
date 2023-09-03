use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3035(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);
    let mut iter = line.split_whitespace();
    let r = iter.next().unwrap().parse::<usize>().unwrap();
    let c = iter.next().unwrap().parse::<usize>().unwrap();
    let zr = iter.next().unwrap().parse::<usize>().unwrap();
    let zc = iter.next().unwrap().parse::<usize>().unwrap();

    let mut image = vec![];
    for _ in 0..r {
        let line = read_line(reader);
        image.push(line);
    }

    (0..r).for_each(|i| {
        (0..zr).for_each(|_| {
            (0..c).for_each(|j| {
                (0..zc).for_each(|_| {
                    write!(writer, "{}", image[i].as_bytes()[j] as char).unwrap();
                });
            });
            writeln!(writer).unwrap();
        });
    });
}

// https://www.acmicpc.net/problem/3035
// 스캐너
#[test]
fn test_solve3035() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "3 3 1 2
.x.
x.x
.x."
            .to_string(),
            want: "..xx..
xx..xx
..xx..
"
            .to_string(),
        },
        TestData {
            s: "3 3 2 1
.x.
x.x
.x."
            .to_string(),
            want: ".x.
.x.
x.x
x.x
.x.
.x.
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3035(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
