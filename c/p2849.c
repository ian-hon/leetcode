#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>

bool isReachableAtTime(int sx, int sy, int fx, int fy, int t) {
    int longest_time = abs(fx - sx);
    if (longest_time < abs(fy - sy)) {
        longest_time = abs(fy - sy);
    }

    if (longest_time == 0) {
        if (t == 1) {
            // cant go outwards and return in 1 step
            return false;
        }
        return true;
    }

    return t >= longest_time;
}

int main() {
    // printf("%d\n", isReachableAtTime(2, 4, 7, 7, 6));
    printf("%d\n", isReachableAtTime(1, 2, 1, 2, 1)); // false
    printf("%d\n", isReachableAtTime(1, 1, 1, 1, 3)); // 
    printf("%d\n", isReachableAtTime(1, 1, 1, 1, 2)); // true
    printf("%d\n", isReachableAtTime(1, 1, 1, 1, 6)); // true
}