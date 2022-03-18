/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLastWord = function(s) {
    const arr = s.split(" ").filter(word => word !== "");
    return arr[arr.length - 1].length
};