pub mod deserializer;
pub mod visitor;

#[cfg(test)]
mod tests {
    use crate::behavioral::visitor::{deserializer::{StringDeserializer, Deserializer, VecDeserializer}, visitor::{TwoValuesStruct, TwoValuesArray}};

    #[test]
    fn test_visitor() {
        let deserializer = StringDeserializer::create(TwoValuesStruct::default());
        let result = deserializer.parse_str("123 456");
        println!("{:?}", result);

        let deserializer = VecDeserializer::create(TwoValuesStruct::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);

        let deserializer = VecDeserializer::create(TwoValuesArray::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);

        println!("Error: {}", deserializer.parse_str("123 456").err().unwrap())
    }
}
