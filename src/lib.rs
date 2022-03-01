use crate::Comparison::{Equal, Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if list_a_is_sublist_of_list_b(_first_list, _second_list) {
        if _first_list.len() == _second_list.len() {
            Equal
        } else {
            Sublist
        }
    } else if list_a_is_sublist_of_list_b(_second_list, _first_list) {
        if _first_list.len() == _second_list.len() {
            Equal
        } else {
            Superlist
        }
    } else {
        Unequal
    }
}

fn list_a_is_sublist_of_list_b<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    if list_a.is_empty() && list_a.len() == list_b.len() {
        return true;
    }

    if list_a.len() > list_b.len() {
        return false;
    }

    let mut list_b_idx = 0;
    let list_a_len = list_a.len();
    let list_b_len = list_b.len();

    while list_b_idx < list_b_len {
        // if the "slice" of elements contains less elements than the list A,
        // list A cannot be a sublist
        if list_b_len - list_b_idx < list_a_len {
            return false;
        }

        let mut list_a_idx = 0;

//        println!("list_b_idx={}", list_b_idx);

        let mut n_items_matched = 0;
        let mut inner_list_b_idx = list_b_idx;
        while list_a_idx < list_a_len && inner_list_b_idx < list_b_len {

            //println!("list_a_idx={} inner_list_b_idx={}", list_a_idx, inner_list_b_idx);

            if list_a[list_a_idx] == list_b[inner_list_b_idx] {
                list_a_idx = list_a_idx + 1;
                inner_list_b_idx = inner_list_b_idx + 1;
                n_items_matched = n_items_matched + 1;
            } else {
                break;
            }
        }

        if n_items_matched == list_a_len {
            return true;
        } else {
            list_b_idx = list_b_idx + 1;
        }
    }

    return false;
}