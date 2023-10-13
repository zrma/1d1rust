use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19583(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);
    let (s, e, q) = {
        let mut iter = line.split_whitespace();
        let s = iter.next().unwrap();
        let e = iter.next().unwrap();
        let q = iter.next().unwrap();
        (str_to_time(s), str_to_time(e), str_to_time(q))
    };

    let mut ans = 0;
    let mut entered = std::collections::HashSet::new();
    let mut exited = std::collections::HashSet::new();

    let mut line = String::new(); // Move the line variable outside the loop

    loop {
        let res = reader.read_line(&mut line);
        if res.is_err() {
            break;
        }

        if line.is_empty() {
            break;
        }

        let (t, name) = {
            let mut iter = line.split_whitespace();
            let t = iter.next().unwrap();
            let name = iter.next().unwrap();
            (str_to_time(t), name)
        };

        if t <= s {
            entered.insert(name.to_string());
        } else if t >= e
            && t <= q
            && entered.contains(&*name.to_string())
            && !exited.contains(&*name.to_string())
        {
            ans += 1;
            exited.insert(name.to_string());
        }

        line.clear(); // Clear the line at the end of each iteration
    }

    write!(writer, "{}", ans).unwrap();
}

fn str_to_time(s: &str) -> i32 {
    let mut iter = s.split(':');
    let h = iter.next().unwrap().parse::<i32>().unwrap();
    let m = iter.next().unwrap().parse::<i32>().unwrap();
    h * 60 + m
}

// https://www.acmicpc.net/problem/19583
// 싸이버개강총회
// noinspection SpellCheckingInspection
#[test]
fn test_solve19583() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "22:00 23:00 23:30
21:30 malkoring
21:33 tolelom
21:34 minjae705
21:35 hhan14
21:36 dicohy27
21:40 906bc
23:00 906bc
23:01 tolelom
23:10 minjae705
23:11 hhan14
23:20 dicohy27"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "06:00 12:00 18:00
06:00 shinyo17
06:00 kimchist
06:00 swoon
06:00 kheee512
06:00 Green55
09:00 kimchist
11:59 shinyo17
12:00 kimchist
17:59 swoon
17:59 swoon
18:00 kheee512
18:01 swoon
18:01 Green55
18:01 kheee512
18:01 swoon
18:21 jinius36
18:40 jeongyun1206"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve19583(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
