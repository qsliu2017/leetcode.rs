#[allow(unused)]
struct Solution;

type Mat<T> = Vec<Vec<T>>;

fn mat_mul<T, M>(a: &Mat<T>, b: &Mat<T>, out: &mut Mat<T>, modu: M)
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Rem<M, Output = T>,
    M: Copy,
{
    out.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, cell)| {
            *cell = (0..a.len())
                .map(|k| (((a[i][k]) % modu) * (b[k][j] % modu)) % modu)
                .fold(T::default(), |acc, a| (acc + a) % modu)
        })
    });
}

fn mat_pow(mut a: Mat<i64>, e: i32, modu: i64) -> Mat<i64> {
    let n = a.len();
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        res[i][i] = 1;
    }
    let mut tmp = vec![vec![0; n]; n];

    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            mat_mul(&res, &a, &mut tmp, modu);
            std::mem::swap(&mut res, &mut tmp);
        }
        mat_mul(&a, &a, &mut tmp, modu);
        std::mem::swap(&mut a, &mut tmp);
        e >>= 1;
    }
    res
}

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let modu = 1000000007;
        let states = (1..m).fold(vec![1 << 0, 1 << 1, 1 << 2], |states, _| {
            states
                .into_iter()
                .flat_map(|state| {
                    (match state & 0b111 {
                        0b001 => &[0b010, 0b100],
                        0b010 => &[0b001, 0b100],
                        0b100 => &[0b001, 0b010],
                        _ => unreachable!(),
                    })
                    .into_iter()
                    .map(move |&a| state << 3 | a)
                })
                .collect()
        });
        let n_state = states.len();
        if n == 1 {
            return n_state as _;
        }
        let mat = states
            .iter()
            .map(|&from| {
                states
                    .iter()
                    .map(|&to| if from & to == 0 { 1 } else { 0 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        mat_pow(mat, n - 1, modu)
            .into_iter()
            .map(|row| row.into_iter().fold(0, |acc, a| (acc + a) % modu))
            .fold(0, |acc, a| (acc + a) % modu) as _
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(1, 1, 3), (1, 2, 6), (5, 5, 580986), (2, 37, 478020091)];
        for (m, n, expected) in tt {
            let output = super::Solution::color_the_grid(m, n);
            assert_eq!(expected, output);
        }
    }
}
