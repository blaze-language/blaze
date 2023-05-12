typedef unsigned long size_t;

typedef struct String String;
struct String {
    char* data;
    size_t length;
};

typedef struct String_View String_View;
struct String_View {
    char* data;
    size_t length;
};

#define Node(T) Node__##T
#define Node__definition(T) \
    typedef struct Node(T) Node(T); \
    struct Node(T) { \
        T data; \
        Node(T)* next; \
    };

#define Linked_List(T) Linked_List__##T
#define Linked_List__definition(T) \
    typedef struct Linked_List(T) Linked_List(T); \
    struct Linked_List(T) { \
        Node(T)* head; \
        Node(T)* tail; \
    };

    

int main(void) {
    Node__definition(int)
    Linked_List__definition(int)
    Linked_List(int)* int_list = ((void*)0);

    return 0;
}