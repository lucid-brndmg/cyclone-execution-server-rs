use gumdrop::{Options, ParsingStyle};

#[derive(Debug, Options)]
pub struct CliArgs {
    #[options(help = "specify a configuration JSON file", meta = "FILE")]
    pub config: Option<String>,
    #[options(help = "prevent opening GUI dialogs and no staring browser")]
    pub silence: bool,
    #[options(help = "specify a location for execution temporary files to locate", meta = "DIR")]
    pub working_directory: Option<String>,
    #[options(help = "specify log level: trace | debug | info | warn | error", meta = "LEVEL")]
    pub log_level: Option<String>,

    #[options(help = "print this help message")]
    pub help: bool,

    #[options(no_short, help = "server host", meta = "IP")]
    pub host: Option<String>,
    #[options(no_short, help = "server port", meta = "PORT")]
    pub port: Option<u16>,
    #[options(no_short, help = "execution timeout in ms", meta = "MS")]
    pub timeout: Option<usize>,

    #[options(no_short, help = "disable syntax check for each Cyclone spec")]
    pub no_syntax_check: bool,

    #[options(no_short, help = "disable file clearing after each execution")]
    pub no_clear_files: bool,

    #[options(no_short, help = "disable ANSI colors for logging")]
    pub no_color: bool,

    #[options(free, help = "location of cyclone.jar")]
    pub cyclone_instance: Vec<String>,
}

impl CliArgs {
    pub fn init() -> Self {
        let opts = Self::parse_args_or_exit(ParsingStyle::AllOptions);
        opts
    }
}