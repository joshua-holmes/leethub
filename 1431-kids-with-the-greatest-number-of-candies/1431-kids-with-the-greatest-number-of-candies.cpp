#include <bits/stdc++.h>

class Solution {
public:
    vector<bool> kidsWithCandies(vector<int>& candies, int extraCandies) {
        vector<bool> answer(candies.size());
        int maxCandies = *std::max_element(candies.begin(), candies.end());
        
        for (int i = 0; i < candies.size(); i++) {
            answer[i] = candies[i] + extraCandies >= maxCandies;
        }
        
        return answer;
    }
};