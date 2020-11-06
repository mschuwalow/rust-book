use super::domain::*;
use regex::Regex;
use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    AddUser { user: User, department: Department },
    ListAllInDepartment { department: Department },
    ListAll(),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl FromStr for Command {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, CommandParseError> {
        let re_add_user = Regex::new(r"Add (.*) to (.*)\.").unwrap();
        let re_list_all_in_dep = Regex::new(r"List all users in (.*)\.").unwrap();
        let re_list_all = Regex::new(r"List all users\.").unwrap();
        if re_add_user.is_match(s) {
            let cap = re_add_user.captures_iter(s).next().unwrap();
            let cmd = Command::AddUser {
                user: User(cap[1].to_string()),
                department: Department(cap[2].to_string()),
            };
            Ok(cmd)
        } else if re_list_all_in_dep.is_match(s) {
            let cap = re_list_all_in_dep.captures_iter(s).next().unwrap();
            let cmd = Command::ListAllInDepartment {
                department: Department(cap[1].to_string()),
            };
            Ok(cmd)
        } else if re_list_all.is_match(s) {
            Ok(Command::ListAll())
        } else {
            Err(CommandParseError(format!("Not a valid command: {}", s)))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CommandParseError(String);

impl Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for CommandParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum TermError {
    Os(io::Error),
    Parse(CommandParseError),
}

impl fmt::Display for TermError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TermError::Os(ref e) => Display::fmt(e, f),
            TermError::Parse(ref e) => Display::fmt(e, f),
        }
    }
}

impl error::Error for TermError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TermError::Os(ref e) => Some(e),
            TermError::Parse(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for TermError {
    fn from(err: io::Error) -> Self {
        TermError::Os(err)
    }
}

impl From<CommandParseError> for TermError {
    fn from(err: CommandParseError) -> Self {
        TermError::Parse(err)
    }
}

pub fn read_command() -> Result<Command, TermError> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let num = buffer.trim().parse()?;
    Ok(num)
}

#[test]
fn test_parse_add_sales() {
    let cmd = Command::AddUser {
        user: User("Amir".to_string()),
        department: Department("Sales".to_string()),
    };
    assert_eq!("Add Amir to Sales.".parse::<Command>(), Ok(cmd));
}

#[test]
fn test_parse_list_all_in_dep() {
    let cmd = Command::ListAllInDepartment {
        department: Department("Engineering".to_string()),
    };
    assert_eq!("Get all users in Engineering.".parse::<Command>(), Ok(cmd));
}

#[test]
fn test_parse_list_all() {
    let cmd = Command::ListAll();
    assert_eq!("Get all users.".parse::<Command>(), Ok(cmd));
}
