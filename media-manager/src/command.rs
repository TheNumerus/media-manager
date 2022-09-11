use crate::AppError;
use libmm::db::Database;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    PrintHelp,
    ListMovies,
    Exit,
}

impl Command {
    pub fn try_from_arr(args: &[impl AsRef<str>]) -> Result<Self, AppError> {
        if args.is_empty() {
            return Err(AppError::ArgsParse("Empty argument list".into()));
        }
        let mut args = args.iter();

        match args.next().unwrap().as_ref() {
            "list-movies" => Ok(Self::ListMovies),
            "help" => Ok(Self::PrintHelp),
            "exit" => Ok(Self::Exit),
            _ => Err(AppError::ArgsParse("Unknown command".into())),
        }
    }

    pub fn execute(self, db: &Database) -> Result<(), AppError> {
        match self {
            Self::PrintHelp => Self::print_help(),
            Self::ListMovies => Self::list_movies(),
            Self::Exit => Ok(()),
        }
    }

    fn print_help() -> Result<(), AppError> {
        println!("help");
        Ok(())
    }

    fn list_movies() -> Result<(), AppError> {
        todo!()
    }
}
