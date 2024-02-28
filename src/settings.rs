use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub trait Settings: DeserializeOwned + Default + Debug {
    //
}
