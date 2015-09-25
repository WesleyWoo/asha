
pub fn foo() {
    let v = vec![2, 3, 5];
    println!("{:?}", v);
    let v2 = v;
    println!("{:?}", v2);
    
//    println!("{:?}", v);//Rust禁止我们在移动后使用v

    let int1 = 1;
    let int2 = int1;
    println!("{:?}", int1);
    println!("{:?}", int2);//Rust不会禁止这样的行为，因为i32并没有指向其它数据的指针，对它的拷贝是一个完整的拷贝。
    
    let a1 = [1, 2, 3];
    let a2 = a1;
    println!("{:?}", a1);
    println!("{:?}", a2);
    
    //事实上Rust中的基本类型之间都是完整的拷贝，变量之间不存在数据竞争
    
    
    let value = 1;
    changeValue(value);
    println!("out of scope {:?}", value);
    
    let mut vec1 = vec![2, 3];
    vec1 = changeValue2(vec1);
    println!("{:?}", vec1);
    
    let mut vec2 = vec![2, 3, 4];
    borrow(&mut vec2);
    println!("{:?}", vec2);
    
}

fn changeValue(mut value:i32) {
    value = value + 1;
    println!("in changeValue scope value is {:?}", value);
}

fn changeValue2(value:Vec<i32>) -> Vec<i32>{
    println!("{:?}", value);
    value
}

fn borrow(vec: &mut Vec<i32>) {
    *vec = vec![1, 3, 5];
    println!("{:?}", vec);
}


