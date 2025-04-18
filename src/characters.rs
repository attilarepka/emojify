use std::collections::HashMap;

#[allow(clippy::too_many_lines)]
pub fn map() -> HashMap<&'static str, Vec<Vec<usize>>> {
    HashMap::from([
        (
            " ",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "!",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "\"",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "#",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "$",
            vec![
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
            ],
        ),
        (
            "%",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "&",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "\\",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "'",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "(",
            vec![
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
            ],
        ),
        (
            ")",
            vec![
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "*",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "+",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            ",",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "-",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "/",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "0",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "1",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "2",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "3",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "4",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "5",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "6",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "7",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "8",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "9",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            ",",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            ";",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "=",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "?",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
            ],
        ),
        (
            "@",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "A",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "B",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "C",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "D",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "E",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "F",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "G",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "H",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "I",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "J",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "K",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "L",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "M",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "N",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "O",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "P",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Q",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
            ],
        ),
        (
            "R",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "S",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "T",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "U",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "V",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "W",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "X",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Y",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Z",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "vec![",
            vec![
                vec![0, 0, 0, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 0],
            ],
        ),
        (
            "]",
            vec![
                vec![0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
            ],
        ),
        (
            "^",
            vec![
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "_",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "`",
            vec![
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "{",
            vec![
                vec![0, 0, 0, 0, 1, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 1, 0],
            ],
        ),
        (
            "|",
            vec![
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
            ],
        ),
        (
            "}",
            vec![
                vec![0, 1, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0, 0],
            ],
        ),
        (
            "~",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "А",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Б",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "В",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Г",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ґ",
            vec![
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Д",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Е",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ё",
            vec![
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Є",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Э",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ж",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "З",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "И",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "І",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ї",
            vec![
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Й",
            vec![
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "К",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Л",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "М",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Н",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "О",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "П",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Р",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "С",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Т",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "У",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ф",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Х",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ц",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
            ],
        ),
        (
            "Ч",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ш",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Щ",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0],
            ],
        ),
        (
            "Ы",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ь",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Ю",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "Я",
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ),
    ])
}
