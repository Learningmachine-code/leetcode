pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1:Vec<i32>,nums2:Vec<i32>)->f64{

        let mut merge=Vec::new();
        let middle=(nums1.len()+nums2.len())/2;
        let (mut i,mut j)=(0,0);
        for _ in 0..=middle {
            if i>=nums1.len(){
                merge.push(nums2[j]);
                j+=1;
                continue;
            }
            if j>=nums2.len(){
                // println!("找到j=nums2.len");
                // break;
                merge.push(nums1[i]);
                j+=1;
                continue;
            }
            if nums1[i]<=nums2[j]{
                merge.push(nums1[i]);
                i+=1;
            }else {
                merge.push(nums2[j]);
                j+=1;
            }
        }
        if middle%2==1{
            return merge[middle] as f64;
        }else{
            return ((merge[middle] as f64)+(merge[middle-1] as f64))/2f64;
        }

    }
}