use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2852(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut scores = [0, 0];
    let mut durations = [0, 0];
    let mut prev_score_time = 0;

    for _ in 0..n {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let (team, time) = read_values!(line, usize, String);

        let time_sec =
            time[0..2].parse::<usize>().unwrap() * 60 + time[3..5].parse::<usize>().unwrap();

        calc_durations(&mut durations, &scores, time_sec, prev_score_time);

        prev_score_time = time_sec;
        scores[team - 1] += 1;
    }

    let time_sec = 48 * 60;
    calc_durations(&mut durations, &scores, time_sec, prev_score_time);

    writeln!(writer, "{:02}:{:02}", durations[0] / 60, durations[0] % 60).unwrap();
    write!(writer, "{:02}:{:02}", durations[1] / 60, durations[1] % 60).unwrap();
}

fn calc_durations(
    durations: &mut [usize; 2],
    scores: &[usize; 2],
    time_sec: usize,
    prev_score_time: usize,
) {
    match scores[0].cmp(&scores[1]) {
        std::cmp::Ordering::Greater => {
            durations[0] += time_sec - prev_score_time;
        }
        std::cmp::Ordering::Less => {
            durations[1] += time_sec - prev_score_time;
        }
        _ => {}
    }
}

// https://www.acmicpc.net/problem/2852
// NBA 농구
#[test]
fn test_solve2852() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
1 20:00"
                .to_string(),
            want: "28:00
00:00"
                .to_string(),
        },
        TestData {
            s: "3
1 01:10
2 21:10
2 31:30"
                .to_string(),
            want: "20:00
16:30"
                .to_string(),
        },
        TestData {
            s: "5
1 01:10
1 02:20
2 45:30
2 46:40
2 47:50"
                .to_string(),
            want: "45:30
00:10"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2852(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
