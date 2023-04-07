use std::error::Error;
use std::fmt;
use std::panic::Location;

#[derive(Debug)]
pub struct GenericError {
    details: String,
    location: &'static Location<'static>,
}

impl GenericError {
   pub fn new(msg: &str) -> GenericError {
        
        GenericError{

            details: msg.to_string(),
            location: std::panic::Location::caller()
        }
    }
}


impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{},  {}",self.details,self.location)
    }
}

impl Error for GenericError {
    
    fn description(&self) -> &str {

        &self.details
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        
        Some(self)
    }
}

impl From<serde_json::Error> for GenericError {

    
    #[track_caller]
    fn from(err: serde_json::Error) -> Self {
        GenericError {
            details: err.to_string(),
            location: std::panic::Location::caller(),
        }
    }
    
}

impl From<std::io::Error> for GenericError {
    
    #[track_caller]
    fn from(err: std::io::Error) -> Self {

        GenericError {
            details: err.to_string(),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<std::env::VarError> for GenericError {
    #[track_caller]
    fn from(err: std::env::VarError) -> Self {
        GenericError {
            details: err.to_string(),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<gtk::glib::Error> for GenericError {

    #[track_caller]
    fn from(err: gtk::glib::Error) -> Self {
        GenericError {
            details: err.to_string(),
            location: std::panic::Location::caller(),
        }
    }
}