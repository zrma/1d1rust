use std::cmp::max;

// https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
#[allow(dead_code)]
fn common_child(s1: String, s2: String) -> i32 {
    let l1 = s1.len() + 1;
    let l2 = s2.len() + 1;
    let mut arr = vec![vec![0; l2]; l2];

    for i in 0..l1 {
        arr[0][i] = 0;
        arr[i][0] = 0;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            if c1 == c2 {
                arr[i + 1][j + 1] = arr[i][j] + 1
            } else {
                arr[i + 1][j + 1] = max(arr[i + 1][j], arr[i][j + 1]);
            }
        }
    }
    arr[l1 - 1][l2 - 1]
}

// https://www.hackerrank.com/challenges/common-child/problem
#[test]
fn test_common_child() {
    assert_eq!(common_child("ABCD".to_string(), "ABDC".to_string()), 3);
    assert_eq!(common_child("HARRY".to_string(), "SALLY".to_string()), 2);
    assert_eq!(common_child("AA".to_string(), "BB".to_string()), 0);
    assert_eq!(
        common_child("SHINCHAN".to_string(), "NOHARAAA".to_string()),
        3
    );
    assert_eq!(common_child("ABCDEF".to_string(), "FBDAMN".to_string()), 2);
}
