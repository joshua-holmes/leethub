class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        if (nums.size() == 0) return 0;
        int middle = nums.size() / 2;
        if (target == nums[middle]) {
            return middle;
        } else if (target > nums[middle]) {
            vector<int> lastHalf = vector<int>(nums.begin() + middle + 1, nums.end());
            return 1 + middle + searchInsert(lastHalf, target);
        }
        vector<int> firstHalf = vector<int>(nums.begin(), nums.end() - middle - (nums.size() == 1 ? 1 : 0) );
        return searchInsert(firstHalf, target);
    }
};