use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Node {
    sym: Option<char>,
    children: Vec<Node>,
}

impl Node {
    fn new_empty(sym: Option<char>) -> Node {
        return Node {
            sym: sym,
            children: vec![],
        };
    }
    
    fn from_string(s: &String) -> Node {
        if s.is_empty() {
            return Node {
                sym: None,
                children: Vec::new(),
            };
        } else {
            return Node {
                sym: _char_at(&s, 0),
                children: vec![Node::from_string(&_tail_str(s))],
            };
        }
    }
    
    fn id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        let id = h.finish();
        return format!("node_{}", id);
    }

    fn print_dot(&self) {
        match self.sym {
            Some(sym) => {
                println!("{} [label=\"{}\",shape=plain];", self.id(), sym);
                for nn in &self.children {
                    if nn.sym == None {
                        break;
                    }
                    println!("{} -> {};", self.id(), &nn.id());
                    // recursively print
                    nn.print_dot();
                }
            }
            None => (),
        }
    }
}

struct Trie {
    children: Vec<Node>,
}

impl Trie {
    fn new() -> Trie {
        return Trie { children: vec![] };
    }

    fn insert(&mut self, m: Node) {
        // 同じSymを持つNodeがあるか探索する
        let idx = self.children.iter().position(|n| n.sym == m.sym);
        match idx {
            Some(i) => {
                let found = self.children.remove(i);
                let new_node = _merge_node(found, m);
                self.children.push(new_node);
            }
            None => {
                self.children.push(m);
            }
        }
    }

    fn print_dot(&self) {
        println!("digraph {{\nrankdir=UB;");
        for n in &self.children {
            n.print_dot();
        }
        println!("}}");
    }
}

fn _char_at(s: &String, i: usize) -> Option<char> {
    return s.chars().nth(i);
}

fn _tail_str(s: &String) -> String {
    s.as_str()[1..].to_string() // 無駄っぽい
}

// DEBUG METHOD
fn _describe_node(n: &Node) {
    eprintln!("Node [{:?}]:", n.sym);
    for c in &n.children {
        eprint!(" [{:?}]", c.sym);
    }
    eprintln!();
}

// TODO: 毎回ノードを生成するので効率が悪い．nをdestructiveに上書きしていくようにしたい
// n, mともに同じsymを持っているという前提
fn _merge_node(n: Node, m: Node) -> Node {
    assert_eq!(n.sym, m.sym);

    // n,mの子ノードをsymでグルーピングして1ノードに寄せる
    let mut children_by_sym: HashMap<char, Vec<Node>> = HashMap::new();
    // n,mの子ノードを一括で扱うためにiteratorにして結合する
    let n_m_children = n.children.into_iter().chain(m.children.into_iter());
    for _n in n_m_children {
        match _n.sym {
            Some(sym) => {
                let v = children_by_sym.get_mut(&sym);
                match v {
                    Some(_v) => {
                        _v.push(_n);
                    }
                    None => {
                        // テーブルに新規にsym => [_n]のエントリを追加する
                        children_by_sym.insert(sym, vec![_n]);
                    }
                }
            }
            None => (), // nop
        }
    }

    // 同じsymを持つノードが混ざるのでそれらもマージする
    let mut new_node = Node::new_empty(n.sym);
    for (sym, nds) in children_by_sym {
        let mut acc = Node::new_empty(Some(sym));
        for n in nds {
            acc = _merge_node(acc, n);
        }
        new_node.children.push(acc);
    }

    return new_node;
}

fn main() {
    let mut t = Trie::new();
    let n = Node::from_string(&"windymelt".to_string());
    let m = Node::from_string(&"window".to_string());
    let o = Node::from_string(&"veritas".to_string());
    t.insert(n);
    t.insert(m);
    t.insert(o);
    
    t.print_dot();
}
