#include <stdio.h>
#include <stdlib.h>

int movesToMakeZigzag(int* nums, int numsSize) {
    if ((numsSize == 0) || (numsSize == 1)) {
        return 0;
    }

    int even = 0;
    int odd = 0;

    for (int i = 0; i < numsSize; i++) {
        int lowest = 0;
        if (i == 0) {
            lowest = nums[i + 1];
        } else if (i == (numsSize - 1)) {
            lowest = nums[i - 1];
        } else {
            lowest = nums[i - 1];
            if (nums[i + 1] < lowest) {
                lowest = nums[i + 1];
            }
        }

        if (nums[i] < lowest) {
            continue;
        }

        if ((i & 1) == 0) {
            odd += nums[i] - lowest + 1;
        } else {
            even += nums[i] - lowest + 1;
        }
    }

    if (odd > even) {
        return even;
    }
    return odd;
}

int main() {
    int s = 3;
    int* n = (int*)malloc(s * sizeof(int));
    n[0] = 0;
    n[1] = 1;
    n[2] = 2;

    printf("%d", movesToMakeZigzag(n, s));

    return 0;
}
