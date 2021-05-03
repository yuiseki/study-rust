# study-rust

## Document

- https://www.rust-lang.org/ja
- https://www.rust-lang.org/ja/learn/get-started

----- ----- -----
## Install on Ubuntu 20.04

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

----- ----- -----
## 作業用ディレクトリ＆gitリポジトリを作る

今回は勉強のために一つのディレクトリに複数のrustプロジェクトを置く。

```bash
mkdir study-rust
cd study-rust
git init
touch README.md
echo "target" > .gitignore
git add .
git commit -m "init"
```

----- ----- -----
## Hello worldをやってみる

- https://www.rust-lang.org/ja/learn/get-started#generating-new-project

#### rustプロジェクトの生成

```bash
cargo new hello-rust
cd hello-rust
```

#### rustプロジェクトの実行

```bash
cargo run
```

#### 変更をコミット

```bash
git add .
git commit -m "add hello-rust"
```

----- ----- -----
## Packageを使ってみる

- https://www.rust-lang.org/ja/learn/get-started#installing-dependencies
- https://www.rust-lang.org/ja/learn/get-started#a-small-rust-app

#### rustプロジェクトの生成

```bash
cd ..
cargo new hello-ferris
cd hello-ferris
```

#### 依存Packageの追加

edit `hello-ferris/Cargo.toml`

```toml
[dependencies]
ferris-says = "0.2"
```

#### 依存Packageのインストール

```bash
cargo build
```

#### Packageを使う

edit `hello-ferris/src/main.rs`

```rust
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

#### rust プロジェクトの実行

```bash
cargo run
```

#### 変更をコミット

```bash
git add .
git commit -m "add hello-ferris"
```


----- ----- -----
## HTTP serverでも作ってみる

- https://doc.rust-jp.rs/book-ja/ch20-01-single-threaded.html
- https://doc.rust-jp.rs/book-ja/ch20-02-multithreaded.html
- https://qiita.com/sogrnwil/items/42fd032999b39f595324

#### rustプロジェクトの生成

```bash
cd ..
cargo new hello-http --bin
cd hello-http
```


#### git管理対象に追加する

```bash
git add .
git commit -m "add hello-http"
```

#### コードを書く

```rust
use std::io::prelude::*;
use std::thread;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(tcp_stream: TcpStream) {
    let mut stream = std::io::BufReader::new(tcp_stream);
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let _method = params.next();
    let path = params.next();

    match path {
        Some("/") => get_root(stream.get_mut()),
        Some("/hoge") => get_hoge(stream.get_mut()),
        Some("/fuga") => get_fuga(stream.get_mut()),
        _ => get_404(stream.get_mut())
    };
}

fn get_root(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nHello World!";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_hoge(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nhoge";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_fuga(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nfuga";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_404(stream: &mut TcpStream) {
    let res = "HTTP/1.1 404 OK\r\n\r\nNot Found";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

#### rust プロジェクトの実行

```bash
cargo run
```

ブラウザで http://localhost:3000/ を開く

#### 変更をコミット

```bash
git add .
git commit -m "add hello-http"
```
