use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14579(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_students: u32 = read_value(read_line(reader));

    let ans = (1..=num_students)
        .map(|idx| {
            let (score, submit_cnt, submit_time) =
                read_values_as!(read_line(reader), u32, u32, u32);
            (idx, score, submit_cnt, submit_time)
        })
        .min_by(|a, b| {
            let (_, a_score, a_submit_cnt, a_submit_time) = a;
            let (_, b_score, b_submit_cnt, b_submit_time) = b;

            b_score
                .cmp(a_score)
                .then_with(|| a_submit_cnt.cmp(b_submit_cnt))
                .then_with(|| a_submit_time.cmp(b_submit_time))
        })
        .expect("min_by should return a value")
        .0;

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/14579
// 덧셈과 곱셈
#[test]
fn test_solve14579() {
    // 해결한 문제 점수의 총합이 높은 참가자가 더 높은 순위를 가진다.
    // 점수의 총합이 같은 경우, 제출 횟수가 적은 참가자가 더 높은 순위를 가진다.
    // 점수의 총합과 제출 횟수가 같은 경우, 마지막으로 점수를 획득한 문제의 업로드 시간이 빠른 참가자가 더 높은 순위를 가진다.
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "2
620 7 179
300 5 100"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
300 5 100
620 7 179
600 9 150"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3
300 5 100
220 7 179
300 3 150"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3
300 5 100
220 7 179
300 5 150"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14579(&mut reader, &mut writer);

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
