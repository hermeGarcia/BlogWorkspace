use crate::ir::*;

pub trait Visitor {
    fn pre_visit_expr(&mut self, _: &Node<Expr>, _: &State) {}
    fn post_visit_expr(&mut self, _: &Node<Expr>, _: &State) {}
    fn pre_visit_var(&mut self, _: &Node<VarExpr>, _: &State) {}
    fn post_visit_var(&mut self, _: &Node<VarExpr>, _: &State) {}
    fn pre_visit_index(&mut self, _: &Node<IndexExpr>, _: &State) {}
    fn post_visit_index(&mut self, _: &Node<IndexExpr>, _: &State) {}
    fn pre_visit_add(&mut self, _: &Node<AddExpr>, _: &State) {}
    fn post_visit_add(&mut self, _: &Node<AddExpr>, _: &State) {}
    fn pre_visit_stmt(&mut self, _: &Node<Stmt>, _: &State) {}
    fn post_visit_stmt(&mut self, _: &Node<Stmt>, _: &State) {}
    fn pre_visit_declaration(&mut self, _: &Node<DeclarationStmt>, _: &State) {}
    fn post_visit_declaration(&mut self, _: &Node<DeclarationStmt>, _: &State) {}
    fn pre_visit_assign(&mut self, _: &Node<AssignStmt>, _: &State) {}
    fn post_visit_assign(&mut self, _: &Node<AssignStmt>, _: &State) {}
    fn pre_visit_if_else(&mut self, _: &Node<IfElseStmt>, _: &State) {}
    fn post_visit_if_else(&mut self, _: &Node<IfElseStmt>, _: &State) {}
    fn pre_visit_while(&mut self, _: &Node<WhileStmt>, _: &State) {}
    fn post_visit_while(&mut self, _: &Node<WhileStmt>, _: &State) {}
    fn pre_visit_block(&mut self, _: &Node<BlockStmt>, _: &State) {}
    fn post_visit_block(&mut self, _: &Node<BlockStmt>, _: &State) {}
}


pub fn traverse_declaration(stmt: &Node<DeclarationStmt>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_declaration(stmt, state);
    visitor.post_visit_declaration(stmt, state);
}
pub fn traverse_assign(stmt: &Node<AssignStmt>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_assign(stmt, state);
    traverse_expr(stmt.elem.lhe, state, visitor);
    traverse_expr(stmt.elem.rhe, state, visitor);
    visitor.post_visit_assign(stmt, state);

}
pub fn traverse_if_else(stmt: &Node<IfElseStmt>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_if_else(stmt, state);
    traverse_stmt(stmt.elem.if_branch, state, visitor);
    if let Some(else_branch) = stmt.elem.else_branch { 
        traverse_stmt(else_branch, state, visitor);
    }
    visitor.post_visit_if_else(stmt, state);
}
pub fn traverse_while(stmt: &Node<WhileStmt>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_while(stmt, state);
    traverse_expr(stmt.elem.cond, state, visitor);
    traverse_stmt(stmt.elem.body, state, visitor);
    visitor.post_visit_while(stmt, state);
}
pub fn traverse_block(stmt: &Node<BlockStmt>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_block(stmt, state);
    for stmt in &stmt.elem.stmts {
        traverse_stmt(*stmt, state, visitor);
    }
    visitor.post_visit_block(stmt, state);
} 

pub fn traverse_var(expr: &Node<VarExpr>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_var(expr, state);
    visitor.post_visit_var(expr, state);
}
pub fn traverse_index(expr: &Node<IndexExpr>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_index(expr, state);
    traverse_expr(expr.elem.lhe, state, visitor);
    traverse_expr(expr.elem.rhe, state, visitor);
    visitor.post_visit_index(expr, state);
}
pub fn traverse_add(expr: &Node<AddExpr>, state: &State, visitor: &mut dyn Visitor) {
    visitor.pre_visit_add(expr, state);
    traverse_expr(expr.elem.lhe, state, visitor);
    traverse_expr(expr.elem.rhe, state, visitor);
    visitor.post_visit_add(expr, state);
}

pub fn traverse_stmt(stmt_id: StmtID, state: &State, visitor: &mut dyn Visitor) {
    use Stmt::*;
    let statement = &state.statements[stmt_id];
    visitor.pre_visit_stmt(statement, state);
    match &statement.elem {
        Declaration(stmt) => traverse_declaration(stmt, state, visitor),
        Assign(stmt) => traverse_assign(stmt, state, visitor),
        While(stmt) => traverse_while(stmt, state, visitor),
        IfElse(stmt) => traverse_if_else(stmt, state, visitor),
        Block(stmt) => traverse_block(stmt, state, visitor),
    }
    visitor.post_visit_stmt(statement, state);
}

pub fn traverse_expr(expr_id: ExprID, state: &State, visitor: &mut dyn Visitor) {
    use Expr::*;
    let expression = &state.expressions[expr_id];
    visitor.pre_visit_expr(expression, state);
    match &expression.elem {
        Var(expr) => traverse_var(expr, state, visitor),
        Index(expr) => traverse_index(expr, state, visitor),
        Add(expr) => traverse_add(expr, state, visitor),

    }
    visitor.post_visit_expr(expression, state);
}