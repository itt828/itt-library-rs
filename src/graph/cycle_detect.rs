// return (dist to cycle, cycle length)
//
fn cycle_detect(f: &Vec<usize>) -> Vec<(usize, usize)> {
    let n = f.len();
    let mut visited = vec![false; n];
    let mut ret = vec![(0, 0); n];
    let mut idx = vec![n; n];
    let mut st = vec![];
    for mut i in 0..n {
        while !visited[i] {
            idx[i] = st.len();
            st.push(i);
            visited[i] = true;
            i = f[i];
        }
        if ret[i] == (0, 0) {
            let cycle_len = st.len() - idx[i];
            while let Some(v) = st.pop() {
                ret[v] = (0, cycle_len);
                if v == i {
                    break;
                }
            }
        }
        while let Some(v) = st.pop() {
            ret[v] = ret[f[v]];
            ret[v].0 += 1;
        }
    }
    ret
}

#[test]
fn cycle_detect_test() {
    let f = vec![1usize, 2, 3, 4, 0, 6, 2, 2, 9, 10, 9, 3, 12];
    let ml = vec![
        (0, 5),
        (0, 5),
        (0, 5),
        (0, 5),
        (0, 5),
        (2, 5),
        (1, 5),
        (1, 5),
        (1, 2),
        (0, 2),
        (0, 2),
        (1, 5),
        (0, 1),
    ];
    let cd = cycle_detect(&f);
    assert_eq!(ml, cd);
}
