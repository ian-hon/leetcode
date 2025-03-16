#include <math.h>

long long repairCars(int* ranks, int ranksSize, int cars) {
    long long cars = (long long)cars;
    long long left = 0, right = __LONG_LONG_MAX__;

    for (int i = 0; i < ranksSize; i++) {
        if (right > ranks[i]) {
            right = ranks[i];
        }
    }
    right *= cars * cars;

    while (left < right) {
        long long mid = (left + right) / 2;

        int flag = 0;
        long long total = 0;
        for (int i = 0; i < ranksSize; i++) {
            total += sqrt(mid / ranks[i]);
            if (total >= cars) {
                right = mid;
                flag = 1;
            }
        }

        if (flag == 0) {
            left = mid + 1;
        }
    }

    return left;
}

int main() {
    return 0;
}