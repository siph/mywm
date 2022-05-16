/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
#[derive(clap::Parser)]
pub struct Config {
    /// The default terminal that mywm will use
    #[clap(long, env)]
    pub mywm_terminal: String,

    /// The default application launcher that mywm will use
    #[clap(long, env)]
    pub mywm_launcher: String,

    /// The path to the start script that mywm will use
    #[clap(long, env)]
    pub mywm_start_script: String,
}
