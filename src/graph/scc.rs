fn scc(e: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = e.len();
    let mut used = vec![false; n];
    let mut order = vec![];
    fn dfs(from: usize, e: &Vec<Vec<usize>>, used: &mut Vec<bool>, order: &mut Vec<usize>) {
        used[from] = true;
        for &x in &e[from] {
            if !used[x] {
                dfs(x, e, used, order);
            }
        }
        order.push(from);
    }
    fn graph_rev(e: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut r = vec![vec![]; e.len()];
        for i in 0..e.len() {
            for &x in &e[i] {
                r[x].push(i);
            }
        }
        r
    }
    fn dfs_rev(
        from: usize,
        col: usize,
        e: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        color: &mut Vec<usize>,
    ) {
        used[from] = true;
        color[from] = col;
        for &x in &e[from] {
            if !used[x] {
                dfs_rev(x, col, e, used, color);
            }
        }
    }

    for i in 0..n {
        if !used[i] {
            dfs(i, &e, &mut used, &mut order);
        }
    }
    let mut color = vec![1 << 30; n];
    for i in 0..n {
        used[i] = false;
    }
    let ee = graph_rev(&e);
    let mut col = 0;
    order.reverse();
    for i in order {
        if !used[i] {
            dfs_rev(i, col, &ee, &mut used, &mut color);
            col += 1;
        }
    }

    color
}

#[test]
fn scc_test() {
    let n = 6;
    let m = 7;
    let e = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[e[i].0].push(e[i].1);
    }
    let a = scc(&g);
    let mut b = std::collections::HashMap::new();
    for (x, i) in a.iter().zip(0..a.len()) {
        (*b.entry(x).or_insert(vec![])).push(i);
    }
    let mut c = std::collections::HashSet::new();
    for x in b {
        c.insert(x.1);
    }
    assert_eq!(
        c,
        vec![vec![0, 3], vec![1, 4], vec![2], vec![5]]
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
    );
}
