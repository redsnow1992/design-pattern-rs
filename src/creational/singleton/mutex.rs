use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

#[cfg(test)]
mod tests {
    use super::do_a_call;
    use super::ARRAY;

    #[test]
    fn test_singleton() {
        do_a_call();
        do_a_call();
        do_a_call();

        assert_eq!(3, ARRAY.lock().unwrap().len());
        assert_eq!(vec![1, 1, 1], *ARRAY.lock().unwrap());
    }
}
