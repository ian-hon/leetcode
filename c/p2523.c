#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int* sieve_of_erathosthenes(int n, int* s) {
    int* map = (int*)malloc((n + 1) * sizeof(int));

    int* result = (int*)malloc((n + 1) * sizeof(int));

    for (int comp = 2; comp < ((int)sqrt(n) + 1); comp++) {
        if (map[comp] == 1) {
            continue;
        }

        int c = comp * comp;
        while (c <= n) {
            map[c] = 1;
            c += comp;
        }
    }

    int size = 0;

    for (int i = 2; i <= n; i++) {
        if (map[i] == 0) {
            result[size] = i;
            size++;
        }
    }

    *s = size;

    return result;
}

int* closestPrimes(int left, int right, int* returnSize) {
    *returnSize = 2;
    int* result = (int*)malloc(2 * sizeof(int));
    result[0] = -1;
    result[1] = -1;

    int size = 0;
    int* primes = sieve_of_erathosthenes(right, &size);

    if ((size == 0) || (size == 1)) {
        return result;
    }

    int smallest = 10001;

    for (int i = 1; i < size; i++) {
        if (primes[i - 1] < left) {
            continue;
        }

        int d = primes[i] - primes[i - 1];
        if (d < smallest) {
            smallest = d;

            printf("set to %d and %d\n", primes[i - 1], primes[i]);

            result[0] = primes[i - 1];
            result[1] = primes[i];
        }
    }

    return result;
}

int main() {
    int size = 0;
    int* result = closestPrimes(10, 19, &size);
    for (int i = 0; i < size; i++) {
        printf("%d\n", result[i]);
    }
}
