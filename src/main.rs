use std::env;

#[derive(Debug)]
enum CommandParseError {
    WrongCommand,
    TooLessArg,
    TooMuchArg,
    SyntaxError(String),
    TypeParse(String),
}

type CommandParseResult<T> = Result<T, CommandParseError>;

trait Command {
    fn execute(&self) -> String;
    fn debug_msg(&self) -> String;
}
fn check_name_and_len(
    first: &str,
    name: &str,
    len: usize,
    min_len: usize,
    max_len: Option<usize>,
) -> CommandParseResult<()> {
    if len < min_len {
        return Err(CommandParseError::TooLessArg);
    }
    if let Some(max_len) = max_len {
        if len > max_len {
            return Err(CommandParseError::TooMuchArg);
        };
    }
    if !first.eq_ignore_ascii_case(name) {
        return Err(CommandParseError::WrongCommand);
    }
    Ok(())
}

#[derive(Debug)]
struct UnKnowCommand {}

impl Command for UnKnowCommand {
    fn execute(&self) -> String {
        format!("UnKnowCommand")
    }

    fn debug_msg(&self) -> String {
        format!("{:#?}", self)
    }
}
impl TryFrom<Vec<&str>> for UnKnowCommand {
    type Error = CommandParseError;

    fn try_from(_value: Vec<&str>) -> Result<Self, Self::Error> {
        Err(CommandParseError::WrongCommand)
    }
}

#[derive(Debug)]
struct GetCommand {
    key: String,
}

impl Command for GetCommand {
    fn execute(&self) -> String {
        format!("GET {}", self.key)
    }

    fn debug_msg(&self) -> String {
        format!("{:#?}", self)
    }
}

impl TryFrom<Vec<&str>> for GetCommand {
    type Error = CommandParseError;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        check_name_and_len(&value[0], "GET", value.len(), 2, Some(2))?;
        let key = value[1].to_owned();
        Ok(GetCommand { key })
    }
}
#[derive(Debug)]
enum SetExist {
    NX,
    XX,
}
#[derive(Debug)]
enum SetExpire {
    EX(u64),
    PX(u64),
    EXAT(u64),
    PXAT(u64),
    KEEPTTL,
}

#[derive(Default, Debug)]
struct SetCommand {
    key: String,
    value: String,
    exist: Option<SetExist>,
    get: bool,
    expire: Option<SetExpire>,
}

impl SetCommand {
    fn set_exist(&mut self, name: &str) -> CommandParseResult<()> {
        if self.exist.is_some() {
            return Err(CommandParseError::SyntaxError(
                "Too much exist arg".to_string(),
            ));
        };
        match name {
            "NX" => self.exist = Some(SetExist::NX),
            "XX" => self.exist = Some(SetExist::XX),
            _ => {}
        }
        Ok(())
    }
    fn set_expire(&mut self, name: &str, arg: Option<&&str>) -> CommandParseResult<()> {
        if self.expire.is_some() {
            return Err(CommandParseError::SyntaxError(
                "Too much expire arg".to_string(),
            ));
        };
        if arg.is_none() {
            if name == "KEEPTTL" {
                self.expire = Some(SetExpire::KEEPTTL);
                return Ok(());
            } else {
                return Err(CommandParseError::SyntaxError(format!(
                    "{} need one arg",
                    name
                )));
            }
        }
        let arg = arg.unwrap().parse::<u64>();
        match arg {
            Ok(arg) => {
                match name {
                    "EX" => self.expire = Some(SetExpire::EX(arg)),
                    "PX" => self.expire = Some(SetExpire::PX(arg)),
                    "EXAT" => self.expire = Some(SetExpire::EXAT(arg)),
                    "PXAT" => self.expire = Some(SetExpire::PXAT(arg)),
                    _ => {}
                };
            }
            Err(e) => {
                return Err(CommandParseError::TypeParse(format!(
                    "Parse {} arg Error: {}",
                    name,
                    e.to_string()
                )))
            }
        }
        Ok(())
    }
}

impl Command for SetCommand {
    fn execute(&self) -> String {
        format!(
            "{} {} {:?} {:?} {:?}",
            self.key, self.value, self.exist, self.get, self.expire
        )
    }
    fn debug_msg(&self) -> String {
        format!("{:#?}", self)
    }
}
const EXIST_NAME: [&str; 2] = ["NX", "XX"];
const EXPIRE_NAME: [&str; 5] = ["EX", "PX", "EXAT", "PXAT", "KEEPTTL"];
impl TryFrom<Vec<&str>> for SetCommand {
    type Error = CommandParseError;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let len = value.len();
        check_name_and_len(&value[0], "SET", len, 3, Some(7))?;
        let mut command = SetCommand::default();
        command.key = value[1].to_owned();
        command.value = value[2].to_owned();
        let mut i = value.iter().skip(3);
        loop {
            let v = i.next();
            if v.is_none() {
                break;
            }
            let v = v.unwrap();

            if EXIST_NAME.contains(v) {
                command.set_exist(v)?;
            }
            if EXPIRE_NAME.contains(v) {
                command.set_expire(v, i.next())?;
            }
            if *v == "GET" {
                command.get = true;
            }
        }
        Ok(command)
    }
}

fn parse(args: Vec<&str>) -> CommandParseResult<Box<dyn Command>> {
    match args[0].to_uppercase().as_str() {
        "SET" => Ok(Box::new(SetCommand::try_from(args)?)),
        "GET" => Ok(Box::new(GetCommand::try_from(args)?)),
        _ => Ok(Box::new(UnKnowCommand {})),
    }
}

// fn parse_command_str(command_str: &str) -> Vec<&str> {

// }

fn main() {
    let arg_str = env::args().skip(1).collect::<Vec<String>>();
    let args: Vec<&str> = arg_str.iter().map(|x| x.as_str()).collect();
    match parse(args) {
        Ok(command) => {
            println!("{}", command.debug_msg());
            println!("{}", command.execute());
        }
        Err(e) => println!("{:#?}", e),
    }
}
