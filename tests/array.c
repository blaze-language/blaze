#include <stdio.h>

#define Vector(T) Vector__##T
#define Vector__definition(T) \
    typedef struct Vector(T) Vector(T); \
    struct Vector(T) { \
        T* data; \
        size_t length; \
        size_t capacity; \
    };

Vector__definition(int)

int main(void) {
    Vector(int)* int_vector = &(Vector(int)){
        .data = ((void*)0),
        .length = 0,
        .capacity = 0,
    };

    printf("int_vector->length = %zu\n", int_vector->length);
    printf("int_vector->capacity = %zu\n", int_vector->capacity);

    return 0;
}
