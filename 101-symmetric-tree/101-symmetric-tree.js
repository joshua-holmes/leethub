/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isSymmetric = function(root) {
    let left = root.left;
    let right = root.right;
    const checker = (n1, n2) => {
        if (!n1 && !n2) return true;
        if (!n1 || !n2 || n1.left?.val !== n2.right?.val || n1.right?.val !== n2.left?.val) return false;
        return checker(n1.left, n2.right) && checker(n1.right, n2.left)
    }
    if (left?.val !== right?.val) return false;
    return checker(left, right)
};