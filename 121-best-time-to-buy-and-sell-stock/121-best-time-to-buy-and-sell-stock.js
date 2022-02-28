/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    if (prices.length === 1) return 0;
    
    let p = prices[0];
    let q = prices[1];
    let max = 0;
    
    for (let i = 1; i < prices.length; i++) {
        max = Math.max(max, q-p);
        if (p > q) p = q
        q = prices[i + 1]; // make q the next iteration
    }
    
    return max;
};