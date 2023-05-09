#define null ((void *)0)

typedef struct AstNode_Null AstNode_Null;
struct AstNode_Null { void* _; };
typedef struct AstNode_Identifier AstNode_Identifier;
struct AstNode_Identifier { char *name; };

typedef enum AstNode_Kind AstNode_Kind;
enum AstNode_Kind {
    AstNodeKind_Null,
    AstNodeKind_Identifier,
};

typedef union AstNode_Data AstNode_Data;
union AstNode_Data {
    AstNode_Null __0;
    AstNode_Identifier __1;
};

typedef struct AstNode AstNode;
struct AstNode {
    AstNode_Kind kind;
    AstNode_Data data;
};