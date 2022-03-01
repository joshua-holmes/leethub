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
    if (!root) return true;
    const checkMirror = (left, right) => {
        if (!left && !right) return true;
        if (!left || !right || left?.val !== right?.val) return false;
        return checkMirror(left.left, right.right) && checkMirror(left.right, right.left)
    }
    return checkMirror(root.left, root.right)
};