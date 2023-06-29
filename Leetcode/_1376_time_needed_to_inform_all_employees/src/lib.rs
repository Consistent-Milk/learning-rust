pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        fn count_times(
            id: usize,
            manager: &[i32],
            inform_time: &[i32],
            times: &mut [i32],
            chain: &mut Vec<usize>,
        ) {
            let mut mid: usize = id;

            while times[mid] < 0 {
                chain.push(mid);
                mid = manager[mid] as usize;
            }

            let mut time: i32 = times[mid] + inform_time[mid];
            
            while let Some(mid) = chain.pop() {
                times[mid] = time;
                time += inform_time[mid];
            }
        }

        let (n, head_id) = (n as usize, head_id as usize);

        let mut times: Vec<i32> = Vec::with_capacity(n);

        for _i in 0..n {
            times.push(-1);
        }

        times[head_id] = 0;

        let mut chain: Vec<usize> = Vec::with_capacity(n - 1);

        for i in 0..n {
            count_times(i, &manager, &inform_time, &mut times, &mut chain);
        }

        times.into_iter().fold(0, |max, time| max.max(time))
    }
}
