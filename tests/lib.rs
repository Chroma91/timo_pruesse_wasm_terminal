#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::add;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
