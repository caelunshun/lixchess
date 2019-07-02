#include <chess.h>
#include <vector.h>
#include <stdio.h>

int main() {
    Vector vec = vector_init(sizeof(int));
    int *f = malloc(sizeof(int));
    *f = 10;
    vector_push(&vec, f);

    printf("f = %d\n", *(int*) vector_get(&vec, 0));
}

