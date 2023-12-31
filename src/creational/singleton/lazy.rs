use std::sync::Mutex;

use lazy_static::lazy_static;

// Taken from: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//
// Rust doesn't really allow a singleton pattern without `unsafe` because it
// doesn't have a safe mutable global state.
//
// `lazy-static` allows declaring a static variable with lazy initialization
// at first access. It is actually implemented via `unsafe` with `static mut`
// manipulation, however, it keeps your code clear of `unsafe` blocks.
//
// `Mutex` provides safe access to a single object.

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

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
    }
}
