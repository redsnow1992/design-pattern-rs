use self::target::Target;

pub mod adaptee;
pub mod adapter;
pub mod target;

fn call(target: impl Target) {
    println!("'{}'", target.request());
}

#[cfg(test)]
mod tests {
    use crate::structural::adapter::{
        adaptee::SpecificTarget, adapter::TargetAdapter, call, target::OrdinaryTarget,
    };

    #[test]
    fn test_adapter() {
        let target = OrdinaryTarget;

        print!("A compatible target can be directly called: ");
        call(target);

        let adaptee = SpecificTarget;
        println!(
            "Adaptee is incompatible with client: '{}'",
            adaptee.specific_request()
        );

        let adapter = TargetAdapter::new(adaptee);

        print!("But with adapter client can call its method: ");
        call(adapter);
    }
}
