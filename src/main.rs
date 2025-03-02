mod data_structure;

use data_structure::linked_list::LinkedList;

fn main() {
    let mut testlist = LinkedList::<i64>::new();
    testlist.insert_at_head(10);
    testlist.insert_at_head(20);
    testlist.insert_at_head(30);
    println!("insert at head 3 : {}", testlist);

    testlist.insert_at_tail(40);
    testlist.insert_at_tail(50);
    testlist.insert_at_tail(60);
    println!("insert at tail 3 : {}", testlist);

    testlist.insert_at_ith(4, 1);
    println!("insert at 4th : {}", testlist);

    testlist.delete_head();
    println!("delete head : {}", testlist);

    
    testlist.delete_tail();
    println!("delete tail : {}", testlist);

    testlist.delete_ith(3);
    println!("delete ith 3 : {}", testlist);

    println!("get 3 : {:?}", testlist.get(3));

    drop(testlist);
    // println!("drop testlist : {}", testlist);
}