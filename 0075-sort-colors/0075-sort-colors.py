class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        if len(nums) == 1:
            return
        is_sorted = False
        while not is_sorted:
            nums_swapped = False
            print(nums)
            for i, n in enumerate(nums):
                if i == 0:
                    continue
                last_n = nums[i - 1]
                if last_n > n:
                    nums[i] = last_n
                    nums[i - 1] = n
                    if not nums_swapped:
                        nums_swapped = True
            if not nums_swapped:
                is_sorted = True
        