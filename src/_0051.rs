#[allow(unused)]
struct Solution;

struct Bitmap(usize);

impl Bitmap {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn contains(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    pub fn remove(&mut self, i: usize) {
        self.0 &= !(1 << i);
    }

    pub fn put(&mut self, i: usize) {
        self.0 |= 1 << i;
    }
}

struct OccupyBitmaps {
    row: Bitmap,
    col: Bitmap,
    lcross: Bitmap,
    rcross: Bitmap,
}

const N_MAX: usize = 9;

impl OccupyBitmaps {
    pub fn new() -> Self {
        Self {
            row: Bitmap::empty(),
            col: Bitmap::empty(),
            lcross: Bitmap::empty(),
            rcross: Bitmap::empty(),
        }
    }

    pub fn conflict(&self, i: usize, j: usize) -> bool {
        self.row.contains(i)
            || self.col.contains(j)
            || self.lcross.contains(i + j)
            || self.rcross.contains(i + N_MAX - j)
    }

    pub fn put(&mut self, i: usize, j: usize) {
        self.row.put(i);
        self.col.put(j);
        self.lcross.put(i + j);
        self.rcross.put(i + N_MAX - j);
    }

    pub fn remove(&mut self, i: usize, j: usize) {
        self.row.remove(i);
        self.col.remove(j);
        self.lcross.remove(i + j);
        self.rcross.remove(i + N_MAX - j);
    }
}

impl Solution {
    fn put_queen(
        n: usize,
        i: usize,
        state: &mut OccupyBitmaps,
        ans: &mut Vec<Vec<String>>,
        map: &mut Vec<Vec<u8>>,
    ) {
        if i == n {
            ans.push(
                map.iter()
                    .map(|row| unsafe { String::from_utf8_unchecked(row.clone()) })
                    .collect(),
            );
            return;
        }
        for j in 0..n {
            if state.conflict(i, j) {
                continue;
            }
            state.put(i, j);
            map[i][j] = b'Q';
            Self::put_queen(n, i + 1, state, ans, map);
            map[i][j] = b'.';
            state.remove(i, j);
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut state = OccupyBitmaps::new();
        let mut ans = vec![];
        let mut map = vec![vec![b'.'; n]; n];
        Self::put_queen(n, 0, &mut state, &mut ans, &mut map);
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                4,
                vec![
                    vec![".Q..", "...Q", "Q...", "..Q."],
                    vec!["..Q.", "Q...", "...Q", ".Q.."],
                ],
            ),
            (1, vec![vec!["Q"]]),
        ];
        for (n, expected) in tt {
            let output = super::Solution::solve_n_queens(n);
            assert_eq!(expected, output);
        }
    }
}
