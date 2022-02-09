#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use regex::Regex;
use timo_pruesse_wasm_terminal::utils::version::*;

#[test]
fn test_get_version() {
    let regex = Regex::new(r"^v1\.\d\.\d$").unwrap();
    assert!(regex.is_match(&get_version()));
}
