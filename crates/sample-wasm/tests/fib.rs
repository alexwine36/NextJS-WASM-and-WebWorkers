#[cfg(test)]
use sample_wasm::fibonacci;

#[test]
fn fib_test() {
    let result = fibonacci(12);
    assert_eq!(result, 144);
    let result = fibonacci(47);
    assert_eq!(result, 2971215073);
}
