# trie-rust

Construct Trie-tree from STDIN line by line, Output DOT document into STDOUT, in Rust

## Example

```shell
$ cargo run | dot -Tpng > out.png
windy
window
candy
candle
^D
```

`out.png`:

## Interface

Use stdin as text input. Text is parsed line per line. Characters will be all up-cased.

DOT document will be generated into stdout. All messages from program should be streamed into stderr.

```shell
$ cargo build --release
$ echo "foo\nbar" | ./target/release/trie
digraph {
rankdir=UB;
node_93874926701824 [label="F",shape=plain];
node_93874926701824 -> node_93874926701488;
node_93874926701488 [label="O",shape=plain];
node_93874926701488 -> node_93874926701536;
node_93874926701536 [label="O",shape=plain];
node_93874926701856 [label="B",shape=plain];
node_93874926701856 -> node_93874926701680;
node_93874926701680 [label="A",shape=plain];
node_93874926701680 -> node_93874926701728;
node_93874926701728 [label="R",shape=plain];
}
$
```