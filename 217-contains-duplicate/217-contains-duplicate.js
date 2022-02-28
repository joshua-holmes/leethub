/**
 * @param {number[]} nums
 * @return {boolean}
 */
var containsDuplicate = function(nums) {
    const dups = nums.filter((n, i) => {
        const index = nums.indexOf(n);
        return i !== index;
    });
    return !!dups.length;
};