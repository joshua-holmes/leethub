"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        visited = {}
        def clone(node):
            if node is None: return
            if node.val not in visited:
                visited[node.val] = Node(node.val)
                for n in node.neighbors:
                    visited[node.val].neighbors.append(clone(n))
            return visited[node.val]
        return clone(node)