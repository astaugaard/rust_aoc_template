Rust AOC template 

various days under the files src/day{N}.rs

auto fetching of inputs
* you need to set the configuration for this with
  ```
  cargo run -- set-fetch-config --agent "github.com/astaugaard/rust_aoc_template by astaugaard@icloud.com" --oauthkey "insert your oauth key here" --year "year that you are participating in"
  ```
* debugging make sure your clock is set correctly as it will (hopefully) not try to request the input for that day or run the day if the input hasn't yet been released yet.
  * If I messed up here is the line that does that https://github.com/astaugaard/rust_aoc_template/blob/dde7ce7398fc0f58d97ec8373a3271cd80717f0b/src/main.rs#L137
