#[test]
fn iterator_demonstration() {
    let v = vec![1,2,3,];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}
fn main() {
    let v1 = vec![1,2,3,];
    // iterators are lazy, so this has no effect yet
    let v1_iter = v1.iter();
    // the for loop consumes the iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
