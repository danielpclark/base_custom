pub fn unique<T: Clone + PartialEq>(chars: Vec<T>) -> Vec<T> {
  let mut a = chars.clone();
  for x in (0..a.len()).rev() {
    for y in (x+1..a.len()).rev() {
      if a[x] == a[y] {
        a.remove(y);
      }
    }
  }
  a
}
