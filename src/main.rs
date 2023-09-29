use cli_utils_rs::bin_selector;
use std::env;

fn main() {
    bin_selector(env::args());
}
