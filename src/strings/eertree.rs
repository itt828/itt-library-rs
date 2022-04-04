use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    //ノードが示す回文列の長さ
    len: isize,
    //ノードから出ていく辺のうち, c: charでラベリングされたもの
    to: HashMap<char, usize>,
    //suffix link
    slink: usize,
}

impl Node {
    fn new(slink: usize, len: isize) -> Self {
        Self {
            len,
            to: HashMap::new(),
            slink,
        }
    }
    //empty string
    fn zero() -> Self {
        Self {
            len: 0,
            to: HashMap::new(),
            slink: 0,
        }
    }
    //imaginary string
    fn minus_one() -> Self {
        Self {
            len: -1,
            to: HashMap::new(),
            slink: 0,
        }
    }
}
#[derive(Debug)]
pub struct Eertree {
    //文字列
    s: Vec<char>,
    // eertree
    tree: Vec<Node>,

    //最後に追加されたノード
    last_idx: usize,
    //maximum suffix palindromeのあるノード番号
    max_suf: usize,
}
impl Eertree {
    pub fn new() -> Self {
        let tree = vec![Node::minus_one(), Node::zero()];
        Self {
            s: Vec::<char>::new(),
            tree,
            last_idx: 1,
            max_suf: 1,
        }
    }
    pub fn make(s: String) -> Self {
        let mut eertree = Eertree::new();
        for x in s.chars() {
            eertree.add(x);
        }
        eertree
    }
    // 文字列の後ろにc(char)を追加し, この操作によって新たにできた回文の数(0か1)を返す
    pub fn add(&mut self, c: char) -> usize {
        self.s.push(c);
        //cXcとなるための, Xのindexを求める->cur_suf
        let mut cur_suf = self.max_suf;
        loop {
            let former = self.s.len() as isize - 2 - self.tree[cur_suf].len;
            if former >= 0 && self.s[former as usize] == c {
                break;
            }
            cur_suf = self.tree[cur_suf].slink;
        }

        //すでにcXcとなる回文が存在するとき
        if self.tree[cur_suf].to.contains_key(&c) {
            self.max_suf = self.tree[cur_suf].to[&c];
            return 0;
        }
        self.last_idx += 1;
        self.tree[cur_suf].to.insert(c, self.last_idx);
        self.max_suf = self.last_idx;
        let mut new_node = Node::new(1, self.tree[cur_suf].len + 2);
        if new_node.len == 1 {
            new_node.slink = 1;
            self.tree.push(new_node);
            return 1;
        }
        //新しくできたノードからのsuffix_linkを探す
        loop {
            cur_suf = self.tree[cur_suf].slink;
            let former = self.s.len() as isize - 2 - self.tree[cur_suf].len;
            if former >= 0 && self.s[former as usize] == c {
                break;
            }
        }
        new_node.slink = self.tree[cur_suf].to[&c];
        self.tree.push(new_node);

        return 1;
    }

    //回文と回文の長さの組を返す.
    pub fn all_unique_palindrome(&self) -> HashMap<String, isize> {
        let mut map = HashMap::new();
        dfs(self, &mut map, 0, String::new());
        dfs(self, &mut map, 1, String::new());
        fn dfs(eertree: &Eertree, map: &mut HashMap<String, isize>, idx: usize, cur_state: String) {
            for (&ne_c, &ne_idx) in &eertree.tree[idx].to {
                let mut ne_state = cur_state.clone();
                ne_state.push(ne_c);
                dfs(eertree, map, ne_idx, ne_state)
            }
            map.insert(cur_state, eertree.tree[idx].len);
        }

        map
    }
}

#[test]
fn eertree_test() {
    let s = "eertree";
    let mut eertree = Eertree::new();
    for x in s.chars() {
        eertree.add(x);
    }
}
