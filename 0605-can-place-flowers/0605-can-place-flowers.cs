public class Solution {
    public bool CanPlaceFlowers(int[] flowerbed, int n) {
        int prev = 0;
        int total_pottable = 0;
        foreach (int pot in flowerbed) {
            switch (prev) {
                case 0:
                    if (pot == 0) {
                        prev = 2;
                        total_pottable += 1;
                        continue;
                    }
                    break;
                case 2:
                    if (pot == 1) {
                        total_pottable -= 1;
                    }
                    break;
            }
            prev = pot;
        }
        
        return total_pottable >= n;
    }
}