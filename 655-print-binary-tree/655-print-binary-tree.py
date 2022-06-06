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
        res = []
        for i in range(0, height + 1):
            res.append([""] * n)
        
        def addNode(res, r, c, node, direction):
            if direction == 'left':
                newC = c - 2**(height - r - 1)
            else:
                newC = c + 2**(height - r - 1)
            newR = r + 1
            res[newR][newC] = str(node.val)
            if node.left:
                addNode(res, newR, newC, node.left, 'left')
            if node.right:
                addNode(res, newR, newC, node.right, 'right')
        r = 0
        c = int((n - 1) / 2)
        res[r][c] = str(root.val)
        if node.left:
            addNode(res, r, c, root.left, 'left')
        if node.right:
            addNode(res, r, c, root.right, 'right')
        return res