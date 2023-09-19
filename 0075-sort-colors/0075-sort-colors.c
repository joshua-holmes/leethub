void sortColors(int* nums, int numsSize){
    int left = 0;
    int right = numsSize - 1;
    int i = 0;
    
    while (i <= right) {
        if (nums[i] == 0) {
            swap(nums, left, i);
            left++;
            i++;
        } else if (nums[i] == 2) {
            swap(nums, i, right);
            right--;
        } else {
            i++;
        }
    }
}

void swap(int* nums, int left_i, int right_i) {
    int left_n = nums[left_i];
    nums[left_i] = nums[right_i];
    nums[right_i] = left_n;
}