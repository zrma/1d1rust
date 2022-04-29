#[allow(dead_code)]
fn solve2437(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();

    let mut sum = 0;
    for n in arr {
        if sum + 1 < n {
            return sum + 1;
        }
        sum += n;
    }
    sum + 1
}

// https://www.acmicpc.net/problem/2437
#[test]
fn test_solve2437() {
    struct TestData {
        given: Vec<i32>,
        want: i32,
    }
    for data in std::vec![
        TestData {
            given: vec! {3, 1, 6, 2, 7, 30, 1},
            want: 21
        },
        TestData {
            given: vec! {1, 1, 2, 3, 6, 7, 30},
            want: 21
        },
        TestData {
            given: vec! {1},
            want: 2
        },
        TestData {
            given: vec! {2},
            want: 1
        },
        TestData {
            given: vec! {3},
            want: 1
        },
        TestData {
            given: vec! {1, 1},
            want: 3
        },
        TestData {
            given: vec! {1, 2},
            want: 4
        },
        TestData {
            given: vec! {1, 3},
            want: 2
        },
        TestData {
            given: vec! {1, 4},
            want: 2
        },
        TestData {
            given: vec! {2, 4},
            want: 1
        },
        TestData {
            given: vec! {1, 2, 4},
            want: 8
        },
        TestData {
            given: vec! {1, 2, 5},
            want: 4
        },
    ] {
        let got = solve2437(data.given);
        assert_eq!(got, data.want);
    }
}
