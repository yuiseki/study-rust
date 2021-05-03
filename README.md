# study-rust

## Document

- https://www.rust-lang.org/ja
- https://www.rust-lang.org/ja/learn/get-started

## Install on Ubuntu 20.04

```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

## Make temporary workspace

今回は勉強のために一つのディレクトリに複数のrustプロジェクトを置く。

```
mkdir study-rust
cd study-rust
git init
touch README.md
git add .
git commit -m "init"
```

## Hello world

```
cargo new hello-rust
cd hello-rust
cargo run
```

`cargo new` コマンドで生成されたディレクトリはgitリポジトリになっているが、
今回は一つのgitリポジトリに複数のrustプロジェクトを配置したいので、
`.git` ディレクトリを削除する。

```
sudo rm -rf .git
git add .
git commit -m "add hello-rust"
```

## Using package

```
cargo new hello-ferris
cd hello-ferris
```

edit `hello-ferris/Cargo.toml`

```
[dependencies]
ferris-says = "0.2"
```

`cargo build`

edit `hello-ferris/src/main.rs`

```
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello world!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

`cargo run`

```
sudo rm -rf .git
git add .
git commit -m "add hello-ferris"
```
