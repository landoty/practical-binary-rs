#include <stdio.h>
#include <stdlib.h>

int small_rand() {
    int r = rand();
    return r % 5;
}

int main() {
    printf("random: %d", small_rand());
    return 0;
    }

