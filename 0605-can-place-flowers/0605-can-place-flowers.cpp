class Solution {
public:
    bool canPlaceFlowers(vector<int>& flowerbed, int n) {
        int pottable = 0;
        for (int i = 0; i < flowerbed.size() && pottable < n; i++) {
            if (flowerbed[i] == 0) {
                int prev = i > 0 ? flowerbed[i - 1] : -1;
                int next = i < flowerbed.size() - 1 ? flowerbed[i + 1] : -1;
                if (prev < 1 && next < 1) {
                    flowerbed[i] = 1;
                    pottable += 1;
                }
            }
        }
        
        return pottable >= n;
    }
};