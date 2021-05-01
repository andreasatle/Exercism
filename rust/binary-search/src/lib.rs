pub fn find<T: PartialOrd>(array: &[T], key: T) -> Option<usize> {
    let n = array.len();
    if n == 0 {
        return None;
    }
    let mut lb = 0;
    let mut ub = n - 1;

    while lb+1 < ub {
        let m = (lb + ub) / 2;
        if key < array[m] {
            ub = m;
        } else {
            lb = m;
        }
    }

    if key == array[lb] {
        Some(lb)
    } else if key == array[ub] {
        Some(ub)
    } else {
        None
    }
}
