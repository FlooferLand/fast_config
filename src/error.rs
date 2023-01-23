/// For use with fatal errors only
pub enum FatalError {

}

/// For use with recoverable errors only
pub enum RecoverableError {
	
}

/// The main result error type of the crate
/// 
/// [`ConfigError::Fatal`] - Returned when your program has little chances of being recovered (ex: I/O errors) 
pub enum ConfigError {
    Fatal(FatalError),
	Recoverable(RecoverableError)
}
