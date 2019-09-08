struct List<'t> {
    value: i32,
    next: Option<&'t List<'t>>,
}

fn print_list(list: &Option<&List>) {
    match list {
        None => {}
        Some(list) => {
            println!("{}", list.value);
            print_list(&list.next);
        }
    }
}

fn build_and_print_list(iter: &mut std::slice::Iter<i32>, tail: Option<&List>) {
    match iter.next() {
        None => print_list(&tail),
        Some(value) => {
            build_and_print_list(
                iter,
                Some(&List {
                    value: *value,
                    next: tail,
                }),
            );
        }
    }
}

fn main() {
    let values = [5, 23, 0, 17, 6, 7, 10, 3, 1];
    let mut iter = values.iter();
    build_and_print_list(&mut iter, None);
}
