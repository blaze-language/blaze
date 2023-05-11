#define Node(T) Node_##T
#define Node__definition(T) \
    typedef struct Node(T) Node(T); \
    struct Node(T) { \
        T data; \
        Node(T)* next; \
    };

#define LinkedList(T) LinkedList_##T
#define LinkedList__definition(T) \
    typedef struct LinkedList(T) LinkedList(T); \
    struct LinkedList(T) { \
        Node(T)* head; \
        Node(T)* tail; \
    };

int main(void) {
    Node__definition(int)
    LinkedList__definition(int)
    LinkedList(int)* int_list = ((void *)0);

    Node__definition(char)
    LinkedList__definition(char)
    LinkedList(char)* char_list = ((void *)0);

    return 0;
}