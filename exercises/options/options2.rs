// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // xTODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        } 
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // xTODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        loop {
            match optional_integers.pop() {
                Some(integer) => {
                    assert_eq!(integer, cursor);
                    cursor -= 1;
                }
                None => break,
            }
        }

        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        assert_eq!(cursor, 0);
    }
}
