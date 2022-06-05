// V. T.'s stack_overflow.rs
// Which will overflow the stack?
enum Tree {
  Node { value: u64, left: Box<Tree>, right: Box<Tree> },
  Leaf,
}

fn double_all_rec(tree: &mut Tree) {
  if let Tree::Node { value, left, right } = tree {
    *value = *value * 2;
    double_all_rec(left);
    double_all_rec(right);
  }
}

fn double_all_stk(tree: &mut Tree) {
  let mut stack : Vec<&mut Tree> = Vec::new();
  stack.push(tree);
  while let Some(node) = stack.pop() {
    if let Tree::Node { value, left, right } = node {
      *value = *value * 2;
      stack.push(right);
      stack.push(left);
    }
  }
}
