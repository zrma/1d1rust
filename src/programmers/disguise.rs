use std::collections::HashMap;

#[allow(dead_code)]
fn disguise(clothes: Vec<Vec<String>>) -> i32 {
    let mut counts = HashMap::new();
    for cloth in clothes {
        let part = cloth[1].clone();
        let count = counts.entry(part).or_insert(0);
        *count += 1;
    }
    let mut result = 1;
    for (_, count) in counts {
        result *= count + 1
    }
    result - 1
}

// noinspection SpellCheckingInspection
// https://programmers.co.kr/learn/courses/30/lessons/42578?language=go
#[test]
fn test_disguise() {
    struct TestData {
        given: Vec<Vec<String>>,
        want: i32,
    }
    for data in vec![
        TestData {
            given: vec![
                vec!["yellow_hat".to_string(), "headgear".to_string()],
                vec!["blue_sunglasses".to_string(), "eyewear".to_string()],
                vec!["green_turban".to_string(), "headgear".to_string()],
            ],
            want: 5,
        },
        TestData {
            given: vec![
                vec!["crow_mask".to_string(), "face".to_string()],
                vec!["blue_sunglasses".to_string(), "face".to_string()],
                vec!["smoky_makeup".to_string(), "face".to_string()],
            ],
            want: 3,
        },
    ] {
        let got = disguise(data.given);
        assert_eq!(got, data.want);
    }
}
