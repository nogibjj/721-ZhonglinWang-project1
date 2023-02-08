# Zhonglin Wang's project 1


## Install Rust and setup virtualenv

* This project is a Rust project that makes query to Google News using Google News API: https://newsapi.org/ and integrate the API query to CLI application. By using this CLI, we can access to and search for the latest news for our entered string.

## Install Rust on Terminal
Type: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

To configure your current shell, run: `source "$HOME/.cargo/env"`

Create a New project folder for this project and put `main.rs` and `Cargo.toml` under the same folder.

## Create a Google News API
<img width="669" alt="image" src="https://user-images.githubusercontent.com/112585430/217635398-b4b68eca-ccca-44e0-8414-392a1c555c7c.png">

And put it in the section of code

<img width="790" alt="image" src="https://user-images.githubusercontent.com/112585430/217635658-a32fc57d-5516-4c9c-b1d8-95047b064b2e.png">

Then we can run our program by running `cargo run news <query>` and see the result.
<img width="797" alt="image" src="https://user-images.githubusercontent.com/112585430/217636116-bdc4f07d-1a4a-4bd9-b956-1c25c910b891.png">
