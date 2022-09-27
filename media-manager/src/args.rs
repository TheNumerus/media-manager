use crate::command::{Command, CommandParser};
use crate::AppError;
use std::env::Args;

#[derive(Debug, PartialEq, Eq)]
pub enum RunMode {
    SingleCommand(Command),
    Interactive,
}

impl RunMode {
    fn try_from_arr(args: &[impl AsRef<str>]) -> Result<Self, AppError> {
        match args {
            [] => Ok(Self::SingleCommand(Command::PrintHelp)),
            [arg, ..] => match arg.as_ref() {
                "-h" | "--help" => Ok(Self::SingleCommand(Command::PrintHelp)),
                "-i" | "--interactive" => Ok(Self::Interactive),
                _ => Ok(Self::SingleCommand(CommandParser::try_from_arr(args)?)),
            },
        }
    }
}

impl TryFrom<Args> for RunMode {
    type Error = AppError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut args: Vec<_> = args.collect();
        args.remove(0);

        Self::try_from_arr(&args)
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::{AppError, RunMode};

    #[test]
    pub fn try_parse_args() {
        let args = [vec!["-i"], vec!["-h"], vec![], vec!["-a"]];

        let results = [
            Ok(RunMode::Interactive),
            Ok(RunMode::SingleCommand(Command::PrintHelp)),
            Ok(RunMode::SingleCommand(Command::PrintHelp)),
            Err(AppError::ArgsParse("".to_owned())),
        ];

        for (args, result) in args.into_iter().zip(results) {
            assert_eq!(RunMode::try_from_arr(&args), result);
        }
    }
}
