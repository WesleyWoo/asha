
fn main() {
    /* ----------------- 范围（Range）迭代器 ----------------- */
    let mut range = 0..3;
    println!("range is {:?}", range);

    loop {
        let result = range.next();
        if result == None {
            break;
        }else {
            println!("{:?}", result);
        }
    }
    
    /* ----------------- iter迭代器 ----------------- */
    let a = [3, 4, 7];
    let mut iter = a.iter();
    loop {
        let result = iter.next();
        if result == None {
            break;
        }else {
            println!("{:?}", result);
        }
    }
    
    let range_collect = range.collect::<Vec<_>>();
    println!("range_collect is {:?}", range_collect);
    
    let iter_collect = iter.collect::<Vec<_>>();
    println!("iter_collect is {:?}", iter_collect);
    
    
    let stopHere = 0;
}