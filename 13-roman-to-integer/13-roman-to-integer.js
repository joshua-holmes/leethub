/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function(s) {
    const roman = [
        {symbol: "I", value: 1},
        {symbol: "V", value: 5},
        {symbol: "X", value: 10},
        {symbol: "L", value: 50},
        {symbol: "C", value: 100},
        {symbol: "D", value: 500},
        {symbol: "M", value: 1000},
    ];
    // Convert individual symbols to numbers to be added/subtracted
    const numbers = s.split("").map(char => roman.find(numeral => numeral.symbol === char).value);
    let acc = 0;
    for (let i = 0; i < numbers.length; i++) {
        const prev = numbers[i - 1];
        const cur = numbers[i];
        const next = numbers[i + 1];
        // If current value is less than next, do no math. Math will be done next round
        if (!next || cur >= next) {
            // If previous is less than current, subtract.
            if (prev && prev < cur) {
                acc += Math.abs(cur - prev);
            // If previous is the same as current, or more, add.
            } else {
                acc += cur;
            }
        }
    };
    return acc;
};