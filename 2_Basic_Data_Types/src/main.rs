fn main() {
    let _string = "Hello, World!";
    let _integer = 10;
    let _list = [ "abcdefg", "hijklmn", "opq", "rst", "uvw", "xyz" ];

    println!("{}", _string);
    println!("{}", _integer);

    for _item in _list {
        println!("{}", _item);
    }

    // for 语句块现在会隐式调用 .iter() 方法
    // for _item in _list.iter() {
    //     println!("{}", _item);
    // }
}
