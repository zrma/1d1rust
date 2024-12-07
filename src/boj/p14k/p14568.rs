use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14568(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));

    // NOTE - 남규와 영훈의 사탕 차이 2만큼 빼고 나누기
    //        나눈 결과에서 무조건 남규에게 나머지를 주면 됨
    //        택희가 갖는 사탕 i는 2의 배수로 증가하며 카운트
    //
    //        택희 주고, 차이 2만큼 빼고 남은 사탕 = n - i - 2
    //        반으로 나누는 경우 = (n - i - 2) / 2
    //        홀수: (1, 4), (2, 3) = 5 // 2 = 2
    //        짝수: (1, 5), (2, 4), (3, 3) = 6 // 2 = 3
    //
    //        택희가 i개 가져갈 때 경우의 수 = (n - i - 2) / 2 (남규에게 남은 2개 더 주면 되므로)

    let mut ans = 0;
    for i in (2..=n - 2).step_by(2) {
        // 택희의 사탕은 2부터 시작. 남규 영훈의 차이 2만큼 뺀 값까지. 택희 사탕은 짝수만.
        ans += (n - i - 2) / 2; // 남은 사탕을 둘이 나눔
    }

    write!(writer, "{}", ans).expect("write! should work");
}

#[allow(dead_code)]
fn solve14568_iter(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));

    let ans: i32 = (2..=n - 2).step_by(2).map(|i| (n - i - 2) / 2).sum();

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/14568
// 2017 연세대학교 프로그래밍 경진대회
#[test]
fn test_solve14568() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "1176".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve14568(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve14568_iter(&mut reader, &mut writer);

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
}
