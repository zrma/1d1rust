use crate::utils::io::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve30402(reader: &mut impl BufRead, writer: &mut impl Write) {
    let input = (0..15)
        .map(|_| read_values::<String>(reader).join(""))
        .collect::<Vec<_>>()
        .join("");

    let output = input.chars().find_map(|c| match c {
        'w' => Some("chunbae"),
        'b' => Some("nabi"),
        'g' => Some("yeongcheol"),
        _ => None,
    });

    if let Some(name) = output {
        write!(writer, "{}", name).unwrap();
    }
}

// https://www.acmicpc.net/problem/30402
// noinspection SpellCheckingInspection
// 감마선을 맞은 컴퓨터
#[test]
fn test_solve30402() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "p o o y r y p o y r p r r o p
y w w y w r w y w p w w w r y
r w y r w r w w w y r p w w o
r p w w w w w y w w o w o r w
y w w w r o p w o r r w p p w
y y w w w o w p o w r p p o o
p w p w p y o p w w w w p y w
y w y o w o w o o o w o w w p
y o w w y w w w r w o p w w p
p w p y w w o w o r w w p r y
p p w w w w y r w w w y y o w
p w p w w w w o o p o w p w p
y p o y w p w w w w w w r w p
p y r w w w w w o w w p o y w
o r w w y y y w w o o y y r w"
                .to_string(),
            want: "chunbae".to_string(),
        },
        TestData {
            s: "p o o y r y p o y r p r r o p
y b b y b r b y b p b b b r y
r b y r b r b b b y r p b b o
r p b b b b b y b b o b o r b
y b b b r o p b o r r b p p b
y y b b b o b p o b r p p o o
p b p b p y o p b b b b p y b
y b y o b o b o o o b o b b p
y o b b y b b b r b o p b b p
p b p y b b o b o r b b p r y
p p b b b b y r b b b y y o b
p b p b b b b o o p o b p b p
y p o y b p b b b b b b r b p
p y r b b b b b o b b p o y b
o r b b y y y b b o o y y r b"
                .to_string(),
            want: "nabi".to_string(),
        },
        TestData {
            s: "p o o y r y p o y r p r r o p
y g g y g r g y g p g g g r y
r g y r g r g g g y r p g g o
r p g g g g g y g g o g o r g
y g g g r o p g o r r g p p g
y y g g g o g p o g r p p o o
p g p g p y o p g g g g p y g
y g y o g o g o o o g o g g p
y o g g y g g g r g o p g g p
p g p y g g o g o r g g p r y
p p g g g g y r g g g y y o g
p g p g g g g o o p o g p g p
y p o y g p g g g g g g r g p
p y r g g g g g o g g p o y g
o r g g y y y g g o o y y r g"
                .to_string(),
            want: "yeongcheol".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve30402(&mut reader, &mut writer);

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
