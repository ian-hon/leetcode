int maximumCount(int* nums, int numsSize) {
    int negatives = 0;
    int positives = 0;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] > 0) {
            positives += 1;
        } else {
            if (nums[i] < 0) {
                negatives += 1;
            }
        }
    }

    if (negatives > positives) {
        return negatives;
    }
    return positives;
}