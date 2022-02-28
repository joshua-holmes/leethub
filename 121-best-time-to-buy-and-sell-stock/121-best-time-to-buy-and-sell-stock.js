/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    // let maxProf = 0;
    // prices.forEach((p, i) => {
    //     const max = Math.max(...prices.slice(i + 1));
    //     maxProf = Math.max(maxProf, max - p)
    // })
    // return maxProf;
    if(prices.length === 1){
        return 0;
    }
    
    let p = prices[0];
    let q = prices[1];
    let max = 0;
    
    for(let i = 1; i < prices.length; i++){
        max = Math.max(max, q-p); // find the max between our max value and q-p
        
        if(p > q){ // if p > q, make p the new q
            p = q;
        }
        
        q = prices[i+1]; // make q the next iteration
    }
    
    return max;
};