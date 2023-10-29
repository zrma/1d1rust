use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7785(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    let mut set = std::collections::HashSet::new();

    for _ in 0..n {
        let (name, status) = read_values!(read_line(reader), String, String);

        if status == "enter" {
            set.insert(name);
        } else {
            set.remove(&name);
        }
    }

    let mut v = set.into_iter().collect::<Vec<String>>();
    v.sort_by(|a, b| b.cmp(a));

    let output = v.join("\n");

    write!(writer, "{}", output).unwrap();
}

// https://www.acmicpc.net/problem/7785
// 회사에 있는 사람
#[test]
fn test_solve7785() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "4
Baha enter
Askar enter
Baha leave
Artem enter"
            .to_string(),
        want: "Askar
Artem"
            .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7785(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
