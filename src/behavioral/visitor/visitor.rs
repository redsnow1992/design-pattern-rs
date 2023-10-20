pub trait Visitor {
    type Value;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

#[derive(Default, Debug)]
pub struct TwoValuesStruct {
    a: i32,
    b: i32,
}

impl Visitor for TwoValuesStruct {
    type Value = TwoValuesStruct;
    
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValuesStruct {
            a: v[0],
            b: v[1],
        }
    }
}

#[derive(Default, Debug)]
pub struct TwoValuesArray {
    ab: [i32; 2],
}

impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0i32; 2];
        ab[0] = v[0];
        ab[1] = v[1];

        TwoValuesArray {
            ab
        }
    }
}
