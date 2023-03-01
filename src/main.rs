pub mod board;

// use tents_and_trees_lib::test;

fn main() {
    let mut tents_and_trees = board::read_board(&String::from("./sample_board.txt"));
    tents_and_trees.print_board();
    let has_changed = tents_and_trees.check_and_complete_all();
    if has_changed {
        tents_and_trees.print_board();
    }
    let has_changed = tents_and_trees.empty_no_surrounding_trees();
    if has_changed {
        tents_and_trees.print_board();
    }
    let has_changed = tents_and_trees.only_one_place_for_tent();
    if has_changed {
        tents_and_trees.print_board();
    }
    println!("Is complete? {}", tents_and_trees.is_complete());
}
