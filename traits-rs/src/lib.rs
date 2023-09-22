//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/22 15:37:44 Friday
//! brief:

#[allow(dead_code)]
struct FqxData<const W: usize, T> {
    columns: [String; W],
    data: Vec<[T; W]>,
}

#[allow(dead_code)]
impl<const W: usize, T> FqxData<W, T> {
    fn new<S>(columns: [S; W], data: Vec<[T; W]>) -> Self
    where
        S: Into<String>,
    {
        Self {
            columns: columns.map(|s| s.into()),
            data,
        }
    }

    fn iter(self) -> FqxII<W, T> {
        self.into_iter()
    }

    fn iter_ref(&self) -> FqxRefII<W, T> {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> FqxMutRefII<W, T> {
        self.into_iter()
    }
}

// ================================================================================================
// OwnerShip
// ================================================================================================

struct FqxII<const W: usize, T> {
    inner: std::vec::IntoIter<[T; W]>,
}

impl<const W: usize, T> Iterator for FqxII<W, T> {
    type Item = [T; W];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<const W: usize, T> IntoIterator for FqxData<W, T> {
    type Item = [T; W];

    type IntoIter = FqxII<W, T>;

    fn into_iter(self) -> Self::IntoIter {
        FqxII {
            inner: self.data.into_iter(),
        }
    }
}

// ================================================================================================
// Reference
// ================================================================================================

struct FqxRefII<'a, const W: usize, T> {
    inner: &'a [[T; W]],
    index: usize,
}

impl<'a, const W: usize, T> Iterator for FqxRefII<'a, W, T> {
    type Item = &'a [T; W];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len() {
            None
        } else {
            let res = &self.inner[self.index];
            self.index += 1;
            Some(res)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, const W: usize, T> IntoIterator for &'a FqxData<W, T> {
    type Item = &'a [T; W];

    type IntoIter = FqxRefII<'a, W, T>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRefII {
            inner: &self.data,
            index: 0,
        }
    }
}

// ================================================================================================
// Mutable Reference
// ================================================================================================

struct FqxMutRefII<'a, const W: usize, T> {
    inner: &'a mut [[T; W]],
    index: usize,
}

impl<'a, const W: usize, T> Iterator for FqxMutRefII<'a, W, T> {
    type Item = &'a mut [T; W];

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        if i < self.inner.len() {
            let ptr = self.inner.as_mut_ptr();
            unsafe { Some(&mut *ptr.add(i)) }
        } else {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, const W: usize, T> IntoIterator for &'a mut FqxData<W, T> {
    type Item = &'a mut [T; W];

    type IntoIter = FqxMutRefII<'a, W, T>;

    fn into_iter(self) -> Self::IntoIter {
        FqxMutRefII {
            inner: &mut self.data,
            index: 0,
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_lib {
    use itertools::Itertools;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_iter() {
        let mut d = FqxData::new(
            ["c1", "c2", "c3"],
            vec![[1, 2, 3], [3, 2, 1], [1, 1, 1], [2, 2, 3], [4, 5, 4]],
        );

        d.iter_mut().for_each(|r| *r = r.map(|e| e * 10));

        d.iter_ref().for_each(|r| println!("{:?}", r))
    }

    #[test]
    fn test_groupby() {
        let d = FqxData::new(
            ["c1", "c2", "c3"],
            vec![[1, 2, 3], [3, 2, 1], [1, 1, 1], [2, 2, 3], [4, 5, 4]],
        );

        let mut res = HashMap::new();

        d.into_iter()
            .group_by(|r| r[0])
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        println!("{:?}", res);
    }
}
