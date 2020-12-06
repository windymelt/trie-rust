use std::collections::HashMap;
use std::io;

// Trieを構成するノードを表現するstruct
// 1つの文字と，0コ以上の子Nodeを持つことができる
// 文字列をNodeに変換すると，文字数分のNodeが作成され，それらが文字列の先頭から末尾にかけての方向で親子関係になる
// e.g. "text" -> [t] --> [e] --> [x] --> [t] --> []
struct Node {
    // ノード1つが表現する文字
    // 終端を示せるようにOption<char>としている
    sym: Option<char>,
    // ノードにぶら下がっている子ノードを表現するVec
    // Vecは内部的にポインタになっているのでコンパイル時にサイズを決定できる
    children: Vec<Node>,
}

impl Node {
    // 子ノードを持たないNodeを生成する
    fn new_empty(sym: Option<char>) -> Node {
        return Node {
            sym: sym,
            children: vec![],
        };
    }
    
    // StringからNodeを作成する
    fn from_string(s: &String) -> Node {
        // chars()を使うと文字列から1文字ずつ取り出せるイテレータが得られるので，これを_from_iterに渡す
        return Node::_from_iter(&mut s.chars());
    }

    // from_stringの実質的な中身
    // Option<char>が出てくる不思議なイテレータをnext()しながら自分を再帰呼び出しすることで，1文字ずつずらしてNodeを作成できる
    fn _from_iter(i: &mut std::str::Chars) -> Node {
        let c = i.next();
        match c {
            Some(_) => {
                return Node {
                    sym: c,
                    children: vec![Node::_from_iter(i)],
                }
            }
            None => {
                return Node::new_empty(None);
            }
        }
    }
    
    // Nodeの識別子を文字列として返す
    // DOT言語を吐き出す際のノード名として使う
    fn id(&self) -> String {
        // idとしてポインタのアドレスを利用する
        let addr = (self as *const Node) as usize;
        return format!("node_{}", addr);
    }

    // NodeをDOT言語として出力する
    // グラフのノードの定義とエッジの定義が同時に出力される
    fn print_dot(&self) {
        match self.sym {
            Some(sym) => {
                println!("{} [label=\"{}\",shape=plain];", self.id(), sym);
                for nn in &self.children {
                    if nn.sym == None {
                        break;
                    }
                    println!("{} -> {};", self.id(), &nn.id());
                    // 再帰的に子ノードについても呼び出すことでノードにぶら下がった全てのノードが出力される
                    nn.print_dot();
                }
            }
            None => (),
        }
    }
}

// トライ木を表現するstruct
// 別の文字から始まる複数のNodeを束ねただけである
struct Trie {
    children: Vec<Node>,
}

impl Trie {
    // 空のTrieを生成する
    fn new() -> Trie {
        return Trie { children: vec![] };
    }

    // TrieにNodeを挿入する
    fn insert(&mut self, m: Node) {
        // 同じSymを持つNodeがあるか探索する
        let idx = self.children.iter().position(|n| n.sym == m.sym);
        match idx {
            Some(i) => {
                // 同じSymのNodeがあれば，そのNodeとマージして終了
                let found = self.children.remove(i);
                let new_node = _merge_node(found, m);
                self.children.push(new_node);
            }
            None => {
                // 同じSymのNodeがなければ，mを子ノードとして追加して終了
                self.children.push(m);
            }
        }
    }

    // TrieをDOT言語で出力する
    // ヘッダ・フッタを出力し，あとはNode::print_dotに任せる
    fn print_dot(&self) {
        println!("digraph {{\nrankdir=LR;");
        for n in &self.children {
            n.print_dot();
        }
        println!("}}");
    }
}

// DEBUG METHOD
fn _describe_node(n: &Node) {
    eprintln!("Node [{:?}]:", n.sym);
    for c in &n.children {
        eprint!(" [{:?}]", c.sym);
    }
    eprintln!();
}

// 2つのNodeをマージし，新たなNodeを生成する
// TODO: 毎回ノードを生成するので効率が悪い．nをdestructiveに上書きしていくようにしたい
fn _merge_node(n: Node, m: Node) -> Node {
    // n, mともに同じsymを持っているという前提
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

    // EOFに到達するまでstdinから1行ずつ文字列を入力し，Trieに追加していく
    let mut inbuf = String::new(); // buffer
    loop {
        match io::stdin().read_line(&mut inbuf) {
            Ok(0) => break, // EOF
            Ok(_) => {
                // not EOF
                let trimmed = inbuf.trim().to_uppercase().to_string(); // 末尾の改行文字を取り除き，大文字に揃える
                t.insert(Node::from_string(&trimmed));
                inbuf.clear(); // バッファを巻き戻すためにclearする（これがないとどんどん追記されてしまう）
            }
            Err(e) => {
                eprintln!("Error occurred while reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }

    t.print_dot();
}

// TEST SECTION
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_create_single_node() {
        let n = Node::from_string(&"win".to_string());
        assert_eq!(n.sym, Some('w'));
        assert_eq!(n.children.len(), 1);
        assert_eq!(n.children[0].sym, Some('i'));
        assert_eq!(n.children[0].children[0].sym, Some('n'));
    }
    #[test]
    fn can_merge_nodes() {
        let n = Node::from_string(&"win".to_string());
        let m = Node::from_string(&"won".to_string());
        let nm = _merge_node(n, m);
        assert_eq!(nm.sym, Some('w'));
        assert_eq!(nm.children.len(), 2);
        if nm.children[0].sym == Some('i') {
            assert_eq!(nm.children[1].sym, Some('o'));
        } else {
            assert_eq!(nm.children[0].sym, Some('o'));
            assert_eq!(nm.children[1].sym, Some('i'));
        }
        
        assert_eq!(nm.children[1].children[0].sym, Some('n'));
        assert_eq!(nm.children[0].children[0].sym, Some('n'));

        // 同じ文字でも違うノードは違うidになる
        assert_ne!(nm.children[0].children[0].id(), nm.children[1].children[0].id());
    }
}