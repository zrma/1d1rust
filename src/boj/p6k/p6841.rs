use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6841(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];
    loop {
        let s = read_line(reader);
        if s.is_empty() {
            break;
        }
        let ans = match s.trim() {
            "CU" => "see you",
            ":-)" => "I’m happy",
            ":-(" => "I’m unhappy",
            ";-)" => "wink",
            ":-P" => "stick out my tongue",
            "(~.~)" => "sleepy",
            "TA" => "totally awesome",
            "CCC" => "Canadian Computing Competition",
            "CUZ" => "because",
            "TY" => "thank-you",
            "YW" => "you’re welcome",
            "TTYL" => "talk to you later",
            _ => s.trim(),
        };
        answers.push(ans.to_string());

        if s.trim() == "TTYL" {
            break;
        }
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/6841
// noinspection SpellCheckingInspection
// I Speak TXTMSG
#[test]
fn test_solve6841() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "CCC
:-)
SQL
TTYL"
                .to_string(),
            want: "Canadian Computing Competition
I’m happy
SQL
talk to you later"
                .to_string(),
        },
        TestData {
            s: "CU
:-)
:-(
;-)
:-P
(~.~)
TA
CCC
CUZ
TY
YW
TTYL"
                .to_string(),
            want: "see you
I’m happy
I’m unhappy
wink
stick out my tongue
sleepy
totally awesome
Canadian Computing Competition
because
thank-you
you’re welcome
talk to you later"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve6841(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
