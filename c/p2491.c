#include <stdio.h>
#include <stdlib.h>

int compare(const void* a, const void* b) {
    int int_a = *((int*)a);
    int int_b = *((int*)b);
    
    if ( int_a == int_b ) return 0;
    else if ( int_a < int_b ) return -1;
    else return 1;
}

long long dividePlayers(int* skill, int skillSize) {
    qsort(skill, skillSize, sizeof(int), compare);

    int target = skill[0] + skill[skillSize - 1];
    long long total = 0;
    for (int i = 0; i < (skillSize / 2); i++) {
        if ((skill[i] + skill[skillSize - i - 1]) != target) {
            return -1;
        }
        total += skill[i] * skill[skillSize - i - 1];
    }

    return total;
}

int main() {
    int size = 6;
    int* c = (int*)malloc(size * sizeof(size));
    c[0] = 3;
    c[1] = 2;
    c[2] = 5;
    c[3] = 1;
    c[4] = 3;
    c[5] = 4;

    // int size = 4;
    // int* c = (int*)malloc(size * sizeof(size));
    // c[0] = 1;
    // c[1] = 1;
    // c[2] = 2;
    // c[3] = 3;

    printf("%d\n", dividePlayers(c, size));
}
