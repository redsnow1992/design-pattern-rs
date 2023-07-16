pub mod device;
pub mod remotes;

#[cfg(test)]
mod tests {
    use super::{
        remotes::{Remote, basic::BasicRemote, advanced::AdvancedRemote, HasMutableDevice},
        device::{Device, tv::Tv, radio::Radio}
    };


    #[test]
    fn test_bridge() {
        test_device(Tv::default());
        test_device(Radio::default());
    }

    fn test_device(device: impl Device + Clone) {
        println!("Tests with basic remote.");
        let mut basic_remote = BasicRemote::new(device.clone());
        basic_remote.power();
        basic_remote.device().print_status();
    
        println!("Tests with advanced remote.");
        let mut advanced_remote = AdvancedRemote::new(device);
        advanced_remote.power();
        advanced_remote.mute();
        advanced_remote.device().print_status();
    }
}