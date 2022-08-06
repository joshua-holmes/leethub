from math import floor
from collections import deque
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0: return 0
        seen = set()
        q = deque([(amount, 0)])
        while q:
            remaining, count = q.pop()
            for coin in coins:
                if remaining - coin == 0:
                    return count + 1
                elif remaining - coin > 0 and remaining - coin not in seen:
                    q.appendleft((remaining - coin, count + 1))
                    seen.add(remaining - coin)
        return -1
        
