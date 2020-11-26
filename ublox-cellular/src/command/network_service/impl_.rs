use super::types::*;
use crate::network::Error;

impl NetworkRegistrationStat {
    pub fn is_access_alive(&self) -> bool {
        match self {
            NetworkRegistrationStat::Registered => true,
            NetworkRegistrationStat::RegisteredRoaming => true,
            _ => false,
        }
    }

    pub fn registration_ok(self) -> Result<Self, Error> {
        match self {
            NetworkRegistrationStat::RegistrationDenied => Err(Error::RegistrationDenied),
            _ => Ok(self),
        }
    }
}
