// enumで、ASTが簡単に表現できる
enum Expression {
  Number(i64),
  Boolean(bool),
  Add(Box<Expression>, Box<Expression>)
}
