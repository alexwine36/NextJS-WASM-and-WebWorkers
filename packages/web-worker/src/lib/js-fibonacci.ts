

export function fibonacci(num: number): number {

    return num < 1 ? 0
    : num <= 2 ? 1
    : fibonacci(num - 1) + fibonacci(num - 2)
}