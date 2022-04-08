



/*
    Statements:
Declaration: let v
assign: e0 := e1
If-else: if (e) s0 (else s1)?
While loop: while (e) s
Block: { s* }
*/

/*
    Expressions:
Var: v
Index:  e0[e1]
Add: e0 + e1
*/


use crate::arena::{Arena, ArenaID};


pub type ExprID = ArenaID<Node<Expr>>;
pub type ExprArena = Arena<Node<Expr>>;
pub type StmtID = ArenaID<Node<Stmt>>;
pub type StmtArena = Arena<Node<Stmt>>;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub start: u8,
    pub end: u8,
}

#[derive(Debug, Clone)]
pub struct Node<Elem> {
    pub metadata: Metadata,
    pub elem: Elem,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Node<VarExpr>),
    Index(Node<IndexExpr>),
    Add(Node<AddExpr>),
}

#[derive(Debug, Clone)]
pub struct VarExpr {
    pub symbol: String
}

#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub lhe: ExprID,
    pub rhe: ExprID, 
}

#[derive(Debug, Clone)]
pub struct AddExpr {
    pub lhe: ExprID,
    pub rhe: ExprID,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declaration(Node<DeclarationStmt>),
    Assign(Node<AssignStmt>),
    IfElse(Node<IfElseStmt>),
    While(Node<WhileStmt>),
    Block(Node<BlockStmt>)
}

#[derive(Debug, Clone)]
pub struct DeclarationStmt {
    pub symbol: String, 
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub lhe: ExprID,
    pub rhe: ExprID,
}

#[derive(Debug, Clone)]
pub struct IfElseStmt {
    pub cond: ExprID,
    pub if_branch: StmtID,
    pub else_branch: Option<StmtID>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: ExprID,
    pub body: StmtID,
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<StmtID>,
} 

#[derive(Debug, Clone)]
pub struct State {
    pub expressions: ExprArena,
    pub statements: StmtArena,
    pub code: Vec<StmtID>,
}