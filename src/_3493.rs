#[allow(unused)]
struct Solution;

struct DisjointUnionSet {
    parent: Vec<usize>,
}

impl DisjointUnionSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: Vec::from_iter(0..n),
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let parent = self.parent[i];
        if parent == i {
            return i;
        }
        let root = self.find(parent);
        self.parent[i] = root;
        root
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let x = self.find(i);
        let y = self.find(j);
        if x == y {
            false
        } else {
            let (x, y) = (x.min(y), x.max(y));
            self.parent[y] = x;
            true
        }
    }
}

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = properties.len();
        let properties = properties
            .into_iter()
            .map(|bits| bits.into_iter().fold(0_u128, |acc, bit| acc | (1 << bit)))
            .collect::<Vec<_>>();
        let mut union_set = DisjointUnionSet::new(n);
        (n - (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .filter(|&(i, j)| (properties[i] & properties[j]).count_ones() >= k as u32)
            .filter(|&(i, j)| union_set.union(i, j))
            .count()) as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![1, 2],
                    vec![1, 1],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 6],
                    vec![7, 7],
                ],
                1,
                3,
            ),
            (vec![vec![1, 2, 3], vec![2, 3, 4], vec![4, 3, 5]], 2, 1),
            (vec![vec![1, 1], vec![1, 1]], 2, 2),
        ];
        for (properties, k, expected) in tt {
            let output = super::Solution::number_of_components(properties, k);
            assert_eq!(expected, output);
        }
    }
}
