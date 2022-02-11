/**
 * @param {number} num
 * @return {string}
 */
var intToRoman = function (num) {
    const legend = [
        { roman: "I", integer: 1 },
        { roman: "V", integer: 5 },
        { roman: "X", integer: 10 },
        { roman: "L", integer: 50 },
        { roman: "C", integer: 100 },
        { roman: "D", integer: 500 },
        { roman: "M", integer: 1000 },
    ].reverse();

    let newNum = num;
    let output = "";
    let i = 0;
    while (newNum) {
        const integer = legend[i].integer
        const numStartsWith9or4 = (String(newNum)[0] === '4' || String(newNum)[0] === '9')
        const intStartsWith5 = String(integer)[0] === '5'
        if (newNum >= integer && !(numStartsWith9or4 && intStartsWith5)) {
            output += legend[i].roman;
            if (numStartsWith9or4) {
                newNum += integer;
            } else {
                newNum -= integer;
            }
            i = -1
        }
        i = (i + 1) % 7;
    }
    return output;
};