use term_size;

cached! {
    WIDTH;
    fn width() -> Option<usize> = {
        term_size::dimensions().map(|tup| tup.0)
    }
}

cached! {
    HEIGHT;
    fn height() -> Option<usize> = {
        term_size::dimensions().map(|tup| tup.1)
    }
}
