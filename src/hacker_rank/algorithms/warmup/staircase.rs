#[allow(dead_code)]
fn staircase(n: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 0..n {
        let s = (0..n)
            .rev()
            .map(|j| if i < j { ' ' } else { '#' })
            .collect();
        res.push(s);
    }
    res
}

// https://www.hackerrank.com/challenges/staircase/problem
#[test]
fn test_staircase() {
    let expected = vec!["     #", "    ##", "   ###", "  ####", " #####", "######"];
    let actual = staircase(6);
    assert_eq!(actual, expected);
}
