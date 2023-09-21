bool canJump(int* nums, int numsSize){
    int farthest = 0, i = 0;
    while (i <= farthest && i < numsSize) {
        if (farthest < nums[i] + i) {
            farthest = nums[i] + i;
        }
        if (farthest >= numsSize - 1)
            return true;
        i += 1;
    }
    return false;
}
