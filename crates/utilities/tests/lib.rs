#[cfg(test)]
use utilities::*;

#[test]
fn add_test() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
