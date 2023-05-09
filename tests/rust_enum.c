// Expr :: enum {
//     Identifier(name: *char),
//     Group(expr: *Expr),
// }
typedef enum Expr_Kind Expr_Kind;
typedef struct Expr_Identifier Expr_Identifier;
typedef struct Expr_Group Expr_Group;
typedef union Expr_Data Expr_Data;
typedef struct Expr Expr;

enum Expr_Kind {
    Expr__Identifier,
    Expr__Group,
};
struct Expr_Identifier { char* name; };
struct Expr_Group { Expr* expr; };
union Expr_Data {
    Expr_Identifier _0;
    Expr_Group _1;
};
struct Expr {
    Expr_Kind kind;
    Expr_Data data;
};
