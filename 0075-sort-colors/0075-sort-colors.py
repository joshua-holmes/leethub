class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        if len(nums) == 1:
            return
        def swap(left_i, right_i):
            left_n = nums[left_i]
            nums[left_i] = nums[right_i]
            nums[right_i] = left_n
        i = 0
        left = 0
        right = len(nums) - 1
        while i <= right:
            if nums[i] == 0:
                swap(left, i)
                left += 1
                i += 1
            elif nums[i] == 2:
                swap(i, right)
                right -= 1
            else:
                i += 1