/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
pub struct Config {
    /// The default terminal that mywm will use
    pub mywm_terminal: String,

    /// The default application launcher that mywm will use
    pub mywm_launcher: String,

    /// The path to the start script that mywm will use
    pub mywm_start_script: String,
}
