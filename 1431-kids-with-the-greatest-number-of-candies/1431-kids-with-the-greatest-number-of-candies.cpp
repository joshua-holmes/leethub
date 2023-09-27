class Solution {
public:
    vector<bool> kidsWithCandies(vector<int>& candies, int extraCandies) {
        vector<bool> answer;
        int maxCandies = *max_element(candies.begin(), candies.end());
        
        for (int i = 0; i < candies.size(); i++) {
            answer.push_back(candies[i] + extraCandies >= maxCandies);
        }
        
        return answer;
    }
};

