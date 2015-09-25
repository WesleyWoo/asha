

/* ----------------- 范围（Range）迭代器 ----------------- */
pub fn fn_range() {
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
}
 /* ----------------- iter迭代器 ----------------- */
pub fn fn_iter() {
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
}

/* ----------------- collect消费者 ----------------- */
pub fn fn_collect() {
    let mut range = 0..3;
    let range_collect = range.collect::<Vec<_>>();
    println!("range_collect is {:?}", range_collect);
    
    let a = [3, 4, 7];
    let mut iter = a.iter();
    let iter_collect = iter.collect::<Vec<_>>();
    println!("iter_collect is {:?}", iter_collect);
}

/* ----------------- map迭代适配器 ----------------- */
pub fn fn_map() {
    let range_for_map = 5..10;
    let map = range_for_map.map(|x| println!("{}", x));
}








