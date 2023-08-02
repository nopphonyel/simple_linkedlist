use linkedlist::LinkedList;

mod linkedlist;

fn main() {
    let mut a_list:LinkedList<i64> = LinkedList::new();
    a_list.push_back(12);
    a_list.push_back(24);
    a_list.push_back(32);
    a_list.print_content();
}
