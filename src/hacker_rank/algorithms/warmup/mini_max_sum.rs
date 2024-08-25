#[allow(dead_code)]
fn mini_max_sum(arr: Vec<i64>) -> (i64, i64) {
    let (mut min, mut max, mut sum) = (i64::MAX, i64::MIN, 0);
    for n in arr {
        if min > n {
            min = n;
        }
        if max < n {
            max = n;
        }
        sum += n
    }

    (sum - max, sum - min)
}

// https://www.hackerrank.com/challenges/mini-max-sum/problem
#[test]
fn test_mini_max_sum() {
    let arr = vec![1, 2, 3, 4, 5];
    let (exclude_max, exclude_min) = mini_max_sum(arr);
    assert_eq!(exclude_max, 10);
    assert_eq!(exclude_min, 14);
}
