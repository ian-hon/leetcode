#include <stdio.h>
#include <string.h>

int minimumRecolors(char* blocks, int k) {
    int length = strlen(blocks);
    int blackCount = 0;
    int highest = 0;

    for (int i = 0; i < k; i++) {
        if (blocks[i] == 'B') {
            highest += 1;
            blackCount += 1;
        }
    }

    printf("start : %d\n", highest);

    int current = highest;
    for (int i = k; i < length; i++) {
        if (blocks[i] == 'B') {
            blackCount += 1;
            current += 1;
        }

        if (blocks[i - k] == 'B') {
            current -= 1;
        }

        if (current > highest) {
            highest = current;
        }
        printf("%c : %c\n", blocks[i - k], blocks[i]);
    }

    printf("highest : %d\n", highest);

    int r = k - highest;

    if (r < 0) {
        return 0;
    }
    return r;
}

int main() {
    // printf("%d\n", minimumRecolors("WBBWWBBWBW", 7));
    // printf("%d\n", minimumRecolors("WBWBBBW", 2));
    printf("%d\n", minimumRecolors("WWBBBWBBBBBWWBWWWB", 16));
    printf("%d\n", minimumRecolors("ABCDEFGHIJKLMNOPQR", 16));
}