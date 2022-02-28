/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    let min = prices[0]
    let maxProf = 0;
    prices.forEach(p => {
        if (p < min) min = p;
        maxProf = Math.max(maxProf, p - min);
    })
    return maxProf;
};