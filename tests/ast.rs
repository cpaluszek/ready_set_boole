use ready_set_boole::{ast::{Ast, AstNode}, LogicalSymbol};

#[test]
fn test_ast_display() {
    let a = AstNode::Operand(LogicalSymbol::True);
    let b = AstNode::Operand(LogicalSymbol::False);
    let or1 = AstNode::Operator(LogicalSymbol::Disjunction, Box::new(a), Box::new(b));

    let d = AstNode::Operand(LogicalSymbol::True);
    let e = AstNode::Operand(LogicalSymbol::False);
    let and1 = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(d), Box::new(e));

    let c = AstNode::Operand(LogicalSymbol::True);
    let or2 = AstNode::Operator(LogicalSymbol::Disjunction, Box::new(c), Box::new(and1));

    let and2 = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(or1), Box::new(or2));

    let ast = Ast::new(and2);

    println!("{}", ast);
}
