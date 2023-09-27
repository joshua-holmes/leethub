class Solution {
public:
    vector<bool> kidsWithCandies(vector<int>& candies, int extraCandies) {
        vector<bool> answer(candies.size(), false);
        int maxCandies = *max_element(candies.begin(), candies.end());
        
        for (int i = 0; i < candies.size(); i++) {
            if (candies[i] + extraCandies >= maxCandies)
                answer[i] = true;
        }
        
        return answer;
    }
};

