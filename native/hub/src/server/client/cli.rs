use clap::{CommandFactory, FromArgMatches, Parser};

#[derive(Clone, Debug)]
pub enum PlaybackMode {
    Sequential,
    RepeatOne,
    RepeatAll,
    Shuffle,
    NoChange,
}

impl std::str::FromStr for PlaybackMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sequential" => Ok(PlaybackMode::Sequential),
            "repeatone" => Ok(PlaybackMode::RepeatOne),
            "repeatall" => Ok(PlaybackMode::RepeatAll),
            "shuffle" => Ok(PlaybackMode::Shuffle),
            "nochange" => Ok(PlaybackMode::NoChange),
            _ => Err(format!("Unknown playback mode: {}", s)),
        }
    }
}

impl From<PlaybackMode> for u32 {
    fn from(val: PlaybackMode) -> Self {
        match val {
            PlaybackMode::Sequential => 0,
            PlaybackMode::RepeatOne => 1,
            PlaybackMode::RepeatAll => 2,
            PlaybackMode::Shuffle => 3,
            PlaybackMode::NoChange => 99,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OperateMode {
    AppendToEnd,
    PlayNext,
    Replace,
}

impl std::str::FromStr for OperateMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "append" | "appendtoend" => Ok(OperateMode::AppendToEnd),
            "next" | "playnext" => Ok(OperateMode::PlayNext),
            "replace" => Ok(OperateMode::Replace),
            _ => Err(format!("Unknown operate mode: {}", s)),
        }
    }
}

impl From<OperateMode> for i32 {
    fn from(val: OperateMode) -> Self {
        match val {
            OperateMode::AppendToEnd => 0,
            OperateMode::PlayNext => 1,
            OperateMode::Replace => 2,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "rune-speaker-client")]
pub enum Command {
    /// List contents of current directory
    Ls {
        /// Use long listing format
        #[arg(short = 'l', default_value_t = false)]
        long: bool,
    },
    /// Print current working directory
    Pwd,
    /// Change directory
    Cd {
        /// Directory to change to
        path: String,
        #[arg(long)]
        id: bool,
    },
    /// Operate playback with mix query
    Opq {
        /// Path to create query from
        path: String,
        /// Playback mode (sequential, repeatone, repeatall, shuffle, nochange)
        #[arg(long, default_value = "nochange")]
        playback_mode: PlaybackMode,
        /// Whether to start playing instantly
        #[arg(long, default_value_t = true)]
        instant_play: bool,
        /// Operation mode (append, next, replace)
        #[arg(long, default_value = "append")]
        operate_mode: OperateMode,
    },
    /// Exit the program
    Quit,
    Exit,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, clap::Error> {
        let input_vec: Vec<String> = std::iter::once("".to_string())
            .chain(shlex::split(input).unwrap_or_default())
            .collect();

        let args = input_vec.iter().map(|s| s.as_str());

        let matches = Command::command()
            .override_usage("> [COMMAND]")
            .disable_help_flag(true)
            .try_get_matches_from(args)?;

        Command::from_arg_matches(&matches)
    }
}
