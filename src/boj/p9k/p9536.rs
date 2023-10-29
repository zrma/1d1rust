use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9536(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let s = read_line(reader);
        let mut sounds = s.split_whitespace().collect::<Vec<&str>>();
        loop {
            let s = read_line(reader);
            if s == "what does the fox say?" {
                break;
            }

            let (_, _, sound) = read_values!(s, String, String, String);
            sounds.retain(|&s| s != sound);
        }

        writeln!(writer, "{}", sounds.join(" ")).unwrap();
    }
}

// https://www.acmicpc.net/problem/9536
// 여우는 어떻게 울지?
#[test]
fn test_solve9536() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "1
toot woof wa ow ow ow pa blub blub pa toot pa blub pa pa ow pow toot
dog goes woof
fish goes blub
elephant goes toot
seal goes ow
what does the fox say?"
            .to_string(),
        want: "wa pa pa pa pa pa pow
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9536(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {}", i);
    }
}
