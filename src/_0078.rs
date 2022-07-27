struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = vec![vec![]];
        nums.iter().for_each(|&v| {
            let mut append = subsets
                .iter()
                .map(|subset| {
                    let mut subset = subset.clone();
                    subset.push(v);
                    subset
                })
                .collect::<Vec<Vec<i32>>>();
            subsets.append(&mut append);
        });
        subsets
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
