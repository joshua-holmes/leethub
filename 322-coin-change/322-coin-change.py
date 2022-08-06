from math import floor

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0: return 0
        cache = {}
        def dfs(total):
            if total in cache:
                return cache[total]
            count = -1
            for coin in coins:
                if total - coin == 0:
                    return 1
                elif total - coin > 0:
                    res = dfs(total - coin)
                    if res == -1:
                        continue
                    if count != -1:
                        count = min(res, count)
                    else:
                        count = res
            output = count + 1 if count != -1 else -1
            cache[total] = output
            return output
        return dfs(amount)
            
        
