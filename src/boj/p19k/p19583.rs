use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19583(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (s, e, q) = {
        let (s, e, q) = read_values_as!(read_line(reader), String, String, String);
        (
            str_to_time(s.as_str()),
            str_to_time(e.as_str()),
            str_to_time(q.as_str()),
        )
    };

    let mut ans = 0;
    let mut entered = std::collections::HashSet::new();
    let mut exited = std::collections::HashSet::new();

    let mut line = String::new(); // Move the line variable outside the loop

    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        let (t, name) = {
            let (t, name) = read_values_as!(line.as_str(), String, String);
            (str_to_time(t.as_str()), name)
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
        line.clear();
    }

    write!(writer, "{}", ans).expect("Failed to write");
}

fn str_to_time(s: &str) -> i32 {
    let mut iter = s.split(':');
    let h: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
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

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
