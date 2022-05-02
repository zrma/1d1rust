use std::cmp::max;
use std::cmp::min;

#[allow(dead_code)]
fn lotto_ranking_min_max(lotto_numbers: Vec<i32>, win_numbers: Vec<i32>) -> Vec<i32> {
    let mut unknown_slots = 0;
    let mut matched_count = 0;
    for lotto_number in lotto_numbers {
        if lotto_number == 0 {
            unknown_slots += 1;
        } else {
            for win_number in &win_numbers {
                if lotto_number == *win_number {
                    matched_count += 1;
                }
            }
        }
    }
    let result = min(6, 6 - (matched_count - 1));
    vec![max(1, result - unknown_slots), result]
}

// https://programmers.co.kr/learn/courses/30/lessons/77484?language=go
#[test]
fn test_lotto_ranking_min_max() {
    struct TestData {
        lotto_numbers: Vec<i32>,
        win_numbers: Vec<i32>,
        want: Vec<i32>,
    }
    for data in std::vec![
        TestData {
            lotto_numbers: vec![44, 1, 0, 0, 31, 25],
            win_numbers: vec![31, 10, 45, 1, 6, 19],
            want: vec![3, 5],
        },
        TestData {
            lotto_numbers: vec![0, 0, 0, 0, 0, 0],
            win_numbers: vec![3, 8, 19, 20, 40, 15, 25],
            want: vec![1, 6],
        },
        TestData {
            lotto_numbers: vec![45, 4, 35, 20, 3, 9],
            win_numbers: vec![20, 9, 3, 45, 4, 35],
            want: vec![1, 1],
        },
    ] {
        let got = lotto_ranking_min_max(data.lotto_numbers, data.win_numbers);
        assert_eq!(got, data.want);
    }
}
