use std::collections::HashMap;

fn main() {
    println!("====== Collection ======");

    // Init Vec
    {
        let v = vec![0; 3];   // 默认值为 0，初始长度为 3
        let v_from = Vec::from([0, 0, 0]);
    }

    /*
     * Vector act like stack. Pop last element.
     * 
     * 如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 创建动态数组，
     * 这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能
     */
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    println!("v[1] = {}", v[1]);
    let x = v.pop();
    println!("x = {:?}", x);
    let first_ele: &i32 = &v[0];
    println!("First element of v is {}", first_ele);

    /* 
     * 还可以使用宏 vec! 来创建数组，与 Vec::new 有所不同，前者能在创建同时给予初始化值
     * 此处的 v 也无需标注类型，编译器只需检查它内部的元素即可自动推导出 v 的类型是 Vec<i32>
     */
    let v = vec![1,2,3];
    for i in &v {
        println!("i = {i}");
    }

    /*
     * 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，
     * 然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存
     */

    // ========================================================================
    // ========================================================================
    // ========================================================================

    // HashMap
    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();
    println!("Have five: {have_five}");

    // * See also: VecDeque,   HashSet,    BTreeMap,
    // *           LinkedList, BinaryHeap, BTreeSet

}
