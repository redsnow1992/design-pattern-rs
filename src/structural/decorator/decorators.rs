use core::str;
use std::{cell::RefCell, rc::Rc};

use super::data_source::DataSource;

pub trait Decorator {
    fn new(ds: Rc<RefCell<dyn DataSource>>) -> Self;
}

pub struct EncryptionDecorator {
    ds: Rc<RefCell<dyn DataSource>>,
}

impl Decorator for EncryptionDecorator {
    fn new(ds: Rc<RefCell<dyn DataSource>>) -> Self {
        Self { ds }
    }
}

impl DataSource for EncryptionDecorator {
    fn write(&mut self, data: String) {
        let mut ds = self.ds.borrow_mut();
        ds.write(self.encode(&data));
    }

    fn read(&self) -> String {
        let ds = self.ds.borrow();
        self.decode(&ds.read())
    }
}

impl EncryptionDecorator {
    fn encode(&self, data: &str) -> String {
        // todo
        data.to_string()
    }

    fn decode(&self, data: &str) -> String {
        // todo
        data.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::structural::decorator::data_source::FileDataSource;

    use super::EncryptionDecorator;

    #[test]
    fn test_decorator() {
        let ds = FileDataSource { name: "test".into() };
        let wrapper = EncryptionDecorator {
            ds: Rc::new(RefCell::new(ds)),
        };
    }
}