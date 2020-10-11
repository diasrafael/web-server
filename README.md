# web-server

A very simple implementation of a multi-thread static webserver using HTTP over TCP. The main goal here is to practice the learnings from the book *The Rust Programing Language*, chapter 20: [Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

## Getting Started

You'll  need to install Rust and Cargo (rust package manager)
* Cargo (Linux and MacOS)
```sh
$ curl https://sh.rustup.rs -sSf | sh
```

### Running

1. Clone the repo
2. Run the server
```sh
cargo run
```

### Usage

1. Open your browser
2. Use one tab to access http://127.0.0.1:7878/sleep, which will open a slow page (10 seconds sleeping thread).
3. Use another tab to access http://127.0.0.1:7878/, which will open a quick page (no sleep).
