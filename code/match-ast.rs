// matchで、ASTを処理するのも簡単
fn eval(expr: Expression) -> i64 {
  match expr {
    Number(n) => n,
    Boolean(tf) => tf as i64,
    Add(lhs, rhs) => eval(*lhs) + eval(*rhs)
  }
}
