use std::cmp::Ordering;

pub trait FindKey<K> {
    fn find<T>(array: T, key: K) -> Option<usize>
    where
        T: AsRef<[K]>;
}

impl FindKey<i32> for [i32] {
    fn find<T>(array: T, key: i32) -> Option<usize>
    where
        T: AsRef<[i32]>,
    {
        if !array.as_ref().contains(&key) {
            return None;
        }

        if array.as_ref().len() == 1 {
            return Some(0);
        }

        let (left, right) = array.as_ref().split_at(array.as_ref().len() / 2);

        let index = match left[left.len() - 1].cmp(&key) {
            Ordering::Greater => <[i32] as FindKey<i32>>::find(left, key).unwrap(),
            Ordering::Less => left.len() + <[i32] as FindKey<i32>>::find(right, key).unwrap(),
            Ordering::Equal => left.len() - 1,
        };

        Some(index)
    }
}

impl<'a> FindKey<&'a str> for [&'a str] {
    fn find<T>(array: T, key: &str) -> Option<usize>
    where
        T: AsRef<[&'a str]>,
    {
        if !array.as_ref().contains(&key) {
            return None;
        }

        if array.as_ref().len() == 1 {
            return Some(0);
        }

        let (left, right) = array.as_ref().split_at(array.as_ref().len() / 2);

        let index = match left[left.len() - 1].cmp(key) {
            Ordering::Greater => <[&str] as FindKey<&str>>::find(left, key).unwrap(),
            Ordering::Less => left.len() + <[&str] as FindKey<&str>>::find(right, key).unwrap(),
            Ordering::Equal => left.len() - 1,
        };

        Some(index)
    }
}

pub fn find<K, T>(array: T, key: K) -> Option<usize>
where
    [K]: FindKey<K>,
    T: AsRef<[K]>,
{
    <[K] as FindKey<K>>::find(array, key)
}