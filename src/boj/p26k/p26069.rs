use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26069(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut set = std::collections::HashSet::new();
    for _ in 0..n {
        let (name0, name1) = read_values_as!(read_line(reader), String, String);

        // noinspection SpellCheckingInspection
        const TARGET: &str = "ChongChong";
        if name0 == TARGET || name1 == TARGET {
            set.insert(name0);
            set.insert(name1);
            continue;
        }

        if set.contains(&name0) {
            set.insert(name1);
        } else if set.contains(&name1) {
            set.insert(name0);
        }
    }

    writeln!(writer, "{}", set.len()).unwrap();
}

// https://www.acmicpc.net/problem/26069
// 붙임성 좋은 총총이
// noinspection SpellCheckingInspection
#[test]
fn test_solve26069() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "12
bnb2011 chansol
chansol chogahui05
chogahui05 jthis
jthis ChongChong
jthis jyheo98
jyheo98 lms0806
lms0806 pichulia
pichulia pjshwa
pjshwa r4pidstart
r4pidstart swoon
swoon tony9402
tony9402 bnb2011"
            .to_string(),
        want: "10".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve26069(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
