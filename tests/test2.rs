#[cfg(test)]
mod test2 {
    extern crate tents_and_trees_lib;

    use tents_and_trees_lib::*;

    #[test]
    fn read_starting_board() {
        let actual_board = read_board(&String::from("./test2.txt"));
        let expected_board = create_starting_board();
        assert_eq!(expected_board, actual_board.get_board_state());
    }

    #[test]
    fn after_update_zeros() {
        let mut actual_board = read_board(&String::from("./test2.txt"));
        let result = actual_board.check_and_complete_all();
        let expected_board = after_zeros_board();
        assert!(result);
        assert_eq!(expected_board, actual_board.get_board_state());
        // Make sure it doesn't update anything if run again.
        let result = actual_board.check_and_complete_all();
        assert!(!result);
        assert_eq!(expected_board, actual_board.get_board_state());
    }

    #[test]
    fn after_update_no_surrounding_trees() {
        let mut actual_board = read_board(&String::from("./test2.txt"));
        actual_board.check_and_complete_all();
        let result = actual_board.empty_no_surrounding_trees();
        let expected_board = after_no_surrounding_trees();
        assert!(result);
        assert_eq!(expected_board, actual_board.get_board_state());
        // Make sure it doesn't update anything if run again.
        let result = actual_board.empty_no_surrounding_trees();
        assert!(!result);
        assert_eq!(expected_board, actual_board.get_board_state());
    }

    #[test]
    fn after_only_space_for_tents() {
        let mut actual_board = read_board(&String::from("./test2.txt"));
        actual_board.check_and_complete_all();
        actual_board.empty_no_surrounding_trees();
        let result = actual_board.only_space_for_tents();
        actual_board.print_board();
        let expected_board = after_only_space_for_tents_board();
        assert!(result);
        assert_eq!(expected_board, actual_board.get_board_state());
    }

    fn create_starting_board() -> String {
        let mut board_string = String::new();
        board_string.push_str(" | 1 3 0 0 2 1 2 \n");
        board_string.push_str("----------------\n");
        board_string.push_str("2|     T     T   \n");
        board_string.push_str("1|       T       \n");
        board_string.push_str("1|     T       T \n");
        board_string.push_str("1|         T     \n");
        board_string.push_str("2| T             \n");
        board_string.push_str("0|               \n");
        board_string.push_str("2|   T     T     \n");
        board_string
    }

    fn after_zeros_board() -> String {
        let mut board_string = String::new();
        board_string.push_str(" | 1 3 0 0 2 1 2 \n");
        board_string.push_str("----------------\n");
        board_string.push_str("2|     T E   T   \n");
        board_string.push_str("1|     E T       \n");
        board_string.push_str("1|     T E     T \n");
        board_string.push_str("1|     E E T     \n");
        board_string.push_str("2| T   E E       \n");
        board_string.push_str("0| E E E E E E E \n");
        board_string.push_str("2|   T E E T     \n");
        board_string
    }

    fn after_no_surrounding_trees() -> String {
        let mut board_string = String::new();
        board_string.push_str(" | 1 3 0 0 2 1 2 \n");
        board_string.push_str("----------------\n");
        board_string.push_str("2| E   T E   T   \n");
        board_string.push_str("1| E E E T       \n");
        board_string.push_str("1| E   T E     T \n");
        board_string.push_str("1|   E E E T     \n");
        board_string.push_str("2| T   E E   E E \n");
        board_string.push_str("0| E E E E E E E \n");
        board_string.push_str("2|   T E E T   E \n");
        board_string
    }

    fn after_only_space_for_tents_board() -> String {
        let mut board_string = String::new();
        board_string.push_str(" | 1 3 0 0 2 1 2 \n");
        board_string.push_str("----------------\n");
        board_string.push_str("2| E X T E   T   \n");
        board_string.push_str("1| E E E T       \n");
        board_string.push_str("1| E X T E     T \n");
        board_string.push_str("1| E E E E T E   \n");
        board_string.push_str("2| T X E E X E E \n");
        board_string.push_str("0| E E E E E E E \n");
        board_string.push_str("2| X T E E T X E \n");
        board_string
    }
}