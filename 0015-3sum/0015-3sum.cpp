using VecNums = std::vector<int>;
using VecAnswer = std::vector<VecNums>;


class Solution {
public:
    VecAnswer threeSum(VecNums& nums) {
        std::sort(nums.begin(), nums.end());

        VecAnswer answer;

        for (int i = 0; i < nums.size(); i++) {
            int front = i + 1;
            int back = nums.size() - 1;
            int target = -nums[i];

            while (front < back) {
                if (nums[front] + nums[back] > target) {
                    back--;
                } else if (nums[front] + nums[back] < target) {
                    front++;
                } else {
                    VecNums triplet = {nums[front], nums[back], nums[i]};
                    answer.push_back(triplet);

                    while (front < back && nums[front] == triplet[0])
                        front++;
                    while (front < back && nums[back] == triplet[1])
                        back--;
                }
            }

            while (i + 1 < nums.size() && nums[i + 1] == nums[i])
                i++;
        }

        return answer; 
    }
};