fn main() {
    let mut arr = [32, 2, 12, 5, 1, 2, 3, 45];
    let l = arr.len();
    let mut vv = 0;

    for v in 0..l {
        vv = v;
        while vv > 0 {
            if arr[vv] < arr[vv - 1] {
                arr.swap(vv, vv - 1);

            }
            vv = vv - 1;
        }

        println!("{}", vv);
    }



    println!("{:?}", arr);


}
