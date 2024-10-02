use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14467(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cows = read_value(read_line(reader));

    #[derive(Copy, Clone)]
    struct CowWaiting {
        arrival: u32,
        duration: u32,
    }

    let mut cow_waiting = (0..num_cows)
        .map(|_| {
            let (arrival, duration) = read_values_as!(read_line(reader), u32, u32);
            CowWaiting { arrival, duration }
        })
        .collect::<Vec<_>>();
    cow_waiting.sort_by_key(|x| x.arrival);

    let mut elapsed: u32 = 0;
    for cow in cow_waiting {
        let CowWaiting { arrival, duration } = cow;
        if elapsed < arrival {
            elapsed = arrival + duration;
        } else {
            elapsed += duration;
        }
    }

    write!(writer, "{}", elapsed).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14467
// noinspection SpellCheckingInspection
// 소가 길을 건너간 이유 3
#[test]
fn test_solve14467() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2 1
8 3
5 7"
            .to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "4
1 1
2 1
3 1
4 1"
            .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14467(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
