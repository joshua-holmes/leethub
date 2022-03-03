/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    const fib = [1, 2];
    if (n === 1) return 1;
    for (let i = 2; i < n; i++) {
        fib.push(fib[i-2] + fib[i-1]);
    }
    return fib[fib.length - 1];
};