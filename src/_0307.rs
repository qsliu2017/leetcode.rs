#[allow(unused)]
struct NumArray {
    bit: Vec<i32>,
}

fn lsb(index: i32) -> i32 {
    index & -index
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            bit: nums
                .iter()
                .enumerate()
                .map(|(index, num)| (index + 1, num))
                .fold(vec![0; nums.len() + 1], |mut bit, (index, &num)| {
                    let mut index = index as i32;
                    while index < bit.len() as _ {
                        bit[index as usize] += num;
                        index += lsb(index);
                    }
                    bit
                }),
        }
    }

    fn get_prefix_sum(&self, index: i32) -> i32 {
        let mut index = index;
        let mut prefix_sum = 0;
        loop {
            prefix_sum += self.bit[index as usize];
            index -= lsb(index);
            if index == 0 {
                break;
            }
        }
        prefix_sum
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut index = index + 1;
        let delta = val - self.get_prefix_sum(index)
            + if index > 0 {
                self.get_prefix_sum(index - 1)
            } else {
                0 as _
            };
        let mut index = index;
        while index < self.bit.len() as _ {
            self.bit[index as usize] += delta;
            index += lsb(index);
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.get_prefix_sum(right + 1) - self.get_prefix_sum(left)
    }
}
