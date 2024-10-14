# eq.1

## my solution 1

```rust
pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1:Vec<i32>,nums2:Vec<i32>)->f64{

        let mut merge=Vec::new();
        let total=nums1.len()+nums2.len();
        let middle=total/2;
        let (mut i,mut j)=(0,0);
        for _ in 0..=middle {
            if i>=nums1.len(){
                merge.push(nums2[j]);
                j+=1;
                continue;
            }
            if j>=nums2.len(){
                merge.push(nums1[i]);
                i+=1;
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
        if middle==0{
            println!("middle=0");
            return merge[0] as f64;
        }
        if total%2==1{
            return merge[middle] as f64;
        }else{
            return ((merge[middle] as f64)+(merge[middle-1] as f64))/2f64;
        }

    }
}
```

---

## my solution 2

```rust

```

