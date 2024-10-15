use a::Solution;
fn main() {
    let nums1=vec![1,2,3,4,5,6];
    let nums2=vec![2,3,4,5,6,7];
    let result=Solution::find_median_sorted_arrays(nums1,nums2);
    println!("{result}");
}
