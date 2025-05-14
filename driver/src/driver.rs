use crate::profile::Profile;

pub trait Driver {
    fn load_profile(&self, profile: &str) -> Result<(), String>;
}