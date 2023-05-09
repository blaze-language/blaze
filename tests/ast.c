typedef struct Expr_Null Expr_Null;
struct Expr_Null { };

typedef struct Expr_Identifier Expr_Identifier;
struct Expr_Identifier { char* name; };

typedef enum Expr_Kind Expr_Kind;
enum Expr_Kind {
    Expr_Kind__Null,
    Expr_Kind__Identifier,
};
static unsigned short const Expr_Kind__values[] = {
    [Expr_Kind__Null] = 0,
    [Expr_Kind__Identifier] = 1,
};

typedef union Expr_Data Expr_Data;
union Expr_Data {
    Expr_Null __0;
    Expr_Identifier __1;
};

typedef struct Expr Expr;
struct Expr {
    Expr_Kind kind;
    Expr_Data data;
};

int main(int argc, char** argv) {
    Expr null_ = (Expr){
        .kind = Expr_Kind__Null,
        .data.__0 = (Expr_Null){},
    };
    Expr ident = (Expr){
        .kind = Expr_Kind__Identifier,
        .data.__1 = (Expr_Identifier){
            .name = "ident",
        },
    };
    return 0;
}