int numberOfAlternatingGroups(int* colors, int colorsSize, int k) {
    int previous = colors[0];

    int current = 0;
    int count = 0;
    for (int i = 1; i < (colorsSize + k - 1); i ++) {
        int n = colors[i < colorsSize ? i : i - colorsSize];

        if (previous != n) {
            current += 1;
            if (current >= (k - 2)) {
                count += 1;
            }
        } else {
            current = 0;
        }
        previous = n;
    }

    return count;
}

int main() {
    return 0;
}