use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Node {
    sym: Option<char>,
    children: Vec<Node>,
}

struct Trie {
    children: Vec<Node>,
}

fn _id(n: &Node) -> String {
    let mut h = DefaultHasher::new();
    n.hash(&mut h);
    let id = h.finish();

    return format!("node_{}", id);
}

fn _char_at(s: &String, i: usize) -> Option<char> {
    return s.chars().nth(i);
}

fn _tail_str(s: &String) -> String {
    s.as_str()[1..].to_string() // 無駄っぽい
}

fn _string_to_node(s: &String) -> Node {
    if s.is_empty() {
        return Node {
            sym: None,
            children: Vec::new(),
        };
    } else {
        return Node {
            sym: _char_at(&s, 0),
            children: vec![_string_to_node(&_tail_str(s))],
        };
    }
}

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
    eprintln!("merging node [{:?}] and [{:?}]", n.sym, m.sym);
    _describe_node(&n);
    _describe_node(&m);

    assert_eq!(n.sym, m.sym);

    // n,mの子ノードをsymでグルーピングして1ノードに寄せる
    let mut children_by_sym: HashMap<char, Vec<Node>> = HashMap::new();
 
    for _n in n.children {
        match _n.sym {
            Some(sym) => {
                let v = children_by_sym.get_mut(&sym);
                eprintln!("gathering {} from node 'n'", sym);
                match v {
                    Some(_v) => {
                        _v.push(_n);
                    }
                    None => {
                        let mut vec: Vec<Node> = Vec::new();
                        vec.push(_n);
                        children_by_sym.insert(sym, vec);
                    }
                }
            }
            None => (), // nop
        }
    }
    for _m in m.children {
        match _m.sym {
            Some(sym) => {
                let v = children_by_sym.get_mut(&sym);
                eprintln!("gathering {} from node 'm'", sym);
                match v {
                    Some(_v) => {
                        _v.push(_m);
                    }
                    None => {
                        let mut vec: Vec<Node> = Vec::new();
                        vec.push(_m);
                        children_by_sym.insert(sym, vec);
                    }
                }
            }
            None => (), // nop
        }
    }

    // 同じsymを持つノードが混ざるのでそれらもマージする

    let mut new_node = Node {
        sym: n.sym,
        children: vec![],
    };
    
    for (sym, nds) in children_by_sym {
        let mut acc = Node { sym: Some(sym), children: vec![]};
        for n in nds {
            acc = _merge_node(acc, n);
        }
        
        new_node.children.push(acc);
    }

    return new_node;
}

fn _insert_trie(t: &mut Trie, m: Node) {
    // 同じSymを持つNodeがあるか探索する
    let idx = t.children.iter().position(|n| n.sym == m.sym);
    match idx {
        Some(i) => {
            let found = t.children.remove(i);
            let new_node = _merge_node(found, m);
            t.children.push(new_node);
        }
        None => {
            t.children.push(m);
        }
    }
}

fn _print_node(n: &Node) {
    match n.sym {
        Some(sym) => {
            println!("{} [label=\"{}\",shape=plain];", _id(n), sym);
            for nn in &n.children {
                if nn.sym == None {
                    break;
                }
                println!("{} -> {};", _id(n), _id(&nn));

                // recursively print
                _print_node(&nn);
            }
        }
        None => ()
    }
    
}

fn _print_trie(t: &Trie) {
    for n in &t.children {
        _print_node(&n);
    }
}

fn main() {
    println!("digraph {{\nrankdir=UB;");
    let mut t = Trie { children: vec![] };
    let n = _string_to_node(&"windymelt".to_string());
    let m = _string_to_node(&"window".to_string());
    let o = _string_to_node(&"veritas".to_string());
    _insert_trie(&mut t, n);
    _insert_trie(&mut t, m);
    _insert_trie(&mut t, o);
    _print_trie(&t);
    println!("}}");
}
