int waysToSplitArray(int* nums, int numsSize) {
    long long left = 0;
    long long right = 0;

    for (int i = 0; i < numsSize; i++) {
        right += nums[i];
    }

    int count = 0;
    for (int i = 0; i < numsSize - 1; i++) {
        left += nums[i];
        right -= nums[i];

        if (left >= right) {
            count++;
        }
    }

    return count;
}

int main() {

}