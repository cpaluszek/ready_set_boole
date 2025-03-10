use ready_set_boole::{ast::{Ast, AstNode}, LogicalSymbol};

#[test]
fn test_ast_display() {
    let a = AstNode::Operand(LogicalSymbol::True);
    let b = AstNode::Operand(LogicalSymbol::False);
    let or1 = AstNode::Operator(LogicalSymbol::Disjunction, Some(Box::new(a)), Some(Box::new(b)));

    let d = AstNode::Operand(LogicalSymbol::True);
    let e = AstNode::Operand(LogicalSymbol::False);
    let and1 = AstNode::Operator(LogicalSymbol::Conjunction, Some(Box::new(d)), Some(Box::new(e)));

    let c = AstNode::Operand(LogicalSymbol::True);
    let or2 = AstNode::Operator(LogicalSymbol::Disjunction, Some(Box::new(c)), Some(Box::new(and1)));

    let and2 = AstNode::Operator(LogicalSymbol::Conjunction, Some(Box::new(or1)), Some(Box::new(or2)));

    let ast = Ast::new(and2);

    println!("{}", ast);
}
