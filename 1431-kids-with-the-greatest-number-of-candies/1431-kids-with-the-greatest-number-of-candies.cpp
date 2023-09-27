int* maxElement(vector<int>::iterator begin, vector<int>::iterator end) {
    int* maxNum = NULL;
    for (vector<int>::iterator itr = begin; itr < end; itr++) {
        if (!maxNum || *maxNum < *itr)
            maxNum = &*itr;
    }
    
    return maxNum;
}
class Solution {
public:
    vector<bool> kidsWithCandies(vector<int>& candies, int extraCandies) {
        vector<bool> answer(candies.size());
        int* maxCandiesPtr = maxElement(candies.begin(), candies.end());
        
        for (int i = 0; i < candies.size(); i++) {
            answer[i] = candies[i] + extraCandies >= *maxCandiesPtr;
        }
        
        return answer;
    }
};

