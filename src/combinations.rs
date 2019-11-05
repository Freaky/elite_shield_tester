/*
A Rust port of `xuniqueSelectionsFromPairs` - originally an algorithm by
Rutger Janssen, optimized and enhanced to work off filtered pairs by
Jamie "Entity" van den Berge - https://github.com/ntt

Ported to Rust by Thomas Hurst - https://github.com/Freaky/
*/

/*
    generates all unique selections of n elements from given set, allowing
    repetitions, with the constraint that any two items in the generated
    selections must appear as pair in allowedPairs in any order. If m is
    specified, each element will appear at most m times in a selection.
*/
pub fn unique_selections_from_pairs<T, P>(
    items: &[T],
    allowed_pairs: &[(T, T)],
    mut n: usize,
    mut m: usize,
    mut task: P,
) where
    T: Sized + PartialEq,
    P: FnMut(&[&T]),
{
    let mut solution: Vec<&T> = vec![&items[0]; n];

    if n == 0 {
        // ask a silly question...
        task(&solution[..]);
        return;
    }

    // sanitize max number of identical items
    if m == 0 || m > n {
        m = n;
    }

    // create bitmask map telling which items are ok to use with which others.
    assert!(items.len() <= 32); // switch to u64 for more items
    let mut allowed: Vec<u32> = vec![0; items.len()];
    for (a, b) in allowed_pairs.iter() {
        let a_idx = items
            .iter()
            .position(|it| it == a)
            .expect("Unexpected allowed item");
        let b_idx = items
            .iter()
            .position(|it| it == b)
            .expect("Unexpected allowed item");
        allowed[a_idx] |= 1 << b_idx;
        allowed[b_idx] |= 1 << a_idx;
    }

    let mut count = vec![0; n]; // tracks how many items processed at each depth

    let mut item_count = vec![0; items.len()]; // individual item count in current solution

    let mut depth = 0; // current solution depth
    n -= 1; // n is now the deepest allowed depth

    // track allowed items at each depth. root level items always allow all items
    let mut allowed_mask = vec![0; n + 1];
    allowed_mask[0] = std::u32::MAX;
    let mut mask = std::u32::MAX;

    let mut idx = 0;
    loop {
        count[depth] = idx;

        // check for counter overflow, carry over 1 if necessary
        while idx == items.len() {
            if depth > 0 {
                depth -= 1;
                idx = count[depth];
                item_count[idx] -= 1;
                idx += 1;
                count[depth] = idx;
                mask = allowed_mask[depth];
                continue;
            }
            // we were at root, and are now done.
            return;
        }

        // see if the current solution so far satisfies the constraints.
        if item_count[idx] == m || mask & (1 << idx) == 0 {
            idx += 1;
            continue;
        }

        if depth == n {
            // generate solutions with the remaining items that are allowed.
            while idx < items.len() {
                if mask & (1 << idx) != 0 {
                    solution[depth] = &items[idx];
                    task(&solution[..]);
                }
                idx += 1;
            }
            continue;
        }

        // update allowed items at next depth
        mask &= allowed[idx];

        // update solution at current depth
        item_count[idx] += 1;
        solution[depth] = &items[idx];

        // go deeper and update the bitmask of allowed items at the new depth.
        depth += 1;
        allowed_mask[depth] = mask;
    }
}
