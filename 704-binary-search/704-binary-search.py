class Solution:
    def search(self, nums: List[int], target: int, startingIndex=0) -> int:
        if not nums:
            return -1
        mid = len(nums) // 2
        if nums[mid] == target:
            return mid + startingIndex
        elif nums[mid] < target:
            return self.search(nums[mid + 1:], target, mid + startingIndex + 1)
        else:
            return self.search(nums[:mid], target, startingIndex)