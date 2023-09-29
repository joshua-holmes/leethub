class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:
        pottable = 0
        prev = 0
        
        for i, pot in enumerate(flowerbed):
            match prev:
                case 0:
                    if pot == 0:
                        pottable += 1
                        prev = 2
                        continue
                case 2:
                    if pot == 1:
                        pottable -= 1
            prev = pot
        
        return pottable >= n
        