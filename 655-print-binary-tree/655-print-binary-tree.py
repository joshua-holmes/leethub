# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    from math import ceil, floor
    def printTree(self, root: Optional[TreeNode]) -> List[List[str]]:
        height = 1
        node = root
        def count(node, level=0):
            left = 0
            right = 0
            if node.left:
                left = count(node.left, level + 1)
            if node.right:
                right = count(node.right, level + 1)
            return max(left, right, level)
        height = count(root)
        n = 2**(height + 1) - 1
        res = [[""] * n for _ in range(0, height + 1)]
        def build(r, c, node):
            if not node:
                return
            res[r][c] = str(node.val)
            build(r + 1, c - 2**(height - r - 1), node.left)
            build(r + 1, c + 2**(height - r - 1), node.right)
        r = 0
        c = (n - 1) // 2
        build(r, c, root)
        return res