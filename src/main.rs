fn main() {
    let mut r = ['a', 'b', 'c', 'd'];
    let len = r.len();
    heap_permutation(&mut r, len, &mut |x| println!("{x:?}"));
}

fn heap_permutation<T>(data: &mut [T], size: usize, callback: &mut dyn FnMut(&[T])) {
    if size == 1 {
        callback(data);
        return;
    }
    for i in 0..size {
        heap_permutation(data, size - 1, callback);
        if size % 2 == 0 {
            data.swap(i, size - 1);
        } else {
            data.swap(0, size - 1);
        }
    }
}
