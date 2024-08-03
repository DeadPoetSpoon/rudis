use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

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
    value: &[&str],
    name: &str,
    min_len: usize,
    max_len: Option<usize>,
) -> CommandParseResult<()> {
    let first = value[0];
    let len = value.len();
    if len < min_len {
        return Err(CommandParseError::TooLessArg);
    }
    if let Some(max_len) = max_len {
        if len > max_len {
            return Err(CommandParseError::TooMuchArg);
        };
    }
    if first != name {
        return Err(CommandParseError::WrongCommand);
    }
    Ok(())
}

#[derive(Debug)]
struct UnKnowCommand {}

impl Command for UnKnowCommand {
    fn execute(&self) -> String {
        "UnKnowCommand".to_string()
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
        check_name_and_len(&value, "GET", 2, Some(2))?;
        let key = value[1].to_owned();
        Ok(GetCommand { key })
    }
}
#[derive(Debug)]
enum SetExist {
    NX,
    XX,
}

#[allow(clippy::upper_case_acronyms)]
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
                    name, e
                )))
            }
        }
        Ok(())
    }
}

impl Command for SetCommand {
    fn execute(&self) -> String {
        format!(
            "SET {} {} {:?} {:?} {:?}",
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
        check_name_and_len(&value, "SET", 3, Some(7))?;
        let mut command = SetCommand {
            key: value[1].to_owned(),
            value: value[2].to_owned(),
            ..Default::default()
        };
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
    match args[0] {
        "SET" => Ok(Box::new(SetCommand::try_from(args)?)),
        "GET" => Ok(Box::new(GetCommand::try_from(args)?)),
        _ => Ok(Box::new(UnKnowCommand {})),
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
        let commands = [
            // get me
            "*2\r\n$3\r\nGET\r\n$2\r\nme\r\n",
            // set me rudis
            "*3\r\n$3\r\nSET\r\n$2\r\nme\r\n$5\r\nrudis\r\n",
            // set me rudis XX
            "*4\r\n$3\r\nSET\r\n$2\r\nme\r\n$5\r\nrudis\r\n$2\r\nXX\r\n",
            // set me rudis PX 2
            "*5\r\n$3\r\nSET\r\n$2\r\nme\r\n$5\r\nrudis\r\n$2\r\nPX\r\n$1\r\n2\r\n",
            // set me rudis and more arg ignore
            "*3\r\n$3\r\nSET\r\n$2\r\nme\r\n$5\r\nrudis\r\n$2\r\nXX\r\n",
            // set me rudis PX will get SyntaxError
            "*4\r\n$3\r\nSET\r\n$2\r\nme\r\n$5\r\nrudis\r\n$2\r\nPX\r\n",
            // UnknowCommand
            "*2\r\n$5\r\nRUDIS\r\n$2\r\nme\r\n",
            // set error 00
            "*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n00\r\n",
            // set error 01 but has something before the start will get right args
            "asdw2*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n01\r\n",
            // set error 02 but has something before the start
            // and something to the end will get right args
            "asdw2*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n02\r\nssss",
            // set error 03 but has something (\rs\n)
            // in the middle will get an error arg start
            "*3\r\n$3\r\nSET\r\n$5\rs\nerror\r\n$2\r\n03\r\n",
            // set error 04 but has something (err [s] or)
            // in the middle will get an error arg start
            "*3\r\n$3\r\nSET\r\n$5\r\nersror\r\n$2\r\n04\r\n",
            // set error 05 but loss something in the middle will get an error arg start
            "*3\r\n$3\r\nSET\r\n$5\r\nerro\r\n$2\r\n05\r\n",
            // set error 06 but loss a \n in the middle will get an error arg start
            "*3\r\n$3\r\nSET\r\n$5\rerror\r\n$2\r\n06\r\n",
            // set error 07 but loss a \r in the middle
            // will get unexpected error such as error arg len or arg len too much
            "*3\r\n$3\r\nSET\r\n$5\nerror\r\n$2\r\n07\r\n",
            // set error 08 but too much args len will get args too much
            // or error arg start while having something behind
            "*4\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n08\r\n",
            // set error 09 but loss too much will get arg len too much
            // but may get [wrong command] args while having something behind
            "*3\r\n$3\r\nSET\r\n$5\r\nerr",
            // set error 10
            "*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n10\r\n",
            // set error 11 but loss every thing behind * will get alone *
            // or may get unexpected error such as args too much
            // or [wrong command] while having something behind
            "*3",
            // set error 12
            "*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n12\r\n",
            // set error 13
            "*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n13\r\n",
            // set error 14 but loss every thing behind $ will get alone $
            // or may get unexpected error such as args too much
            // or [wrong command] while having something behind
            "*3\r\n$3\r\nSET\r\n$2\r\n",
            // set error 15
            "*3\r\n$3\r\nSET\r\n$5\r\nerror\r\n$2\r\n15\r\n",
            // set error 16 will get alone *
            "*vas",
            // set error 17 will skip
            "asdassddfsdf",
        ];
        for c in commands {
            let _ = stream.write(c.as_bytes()).await;
        }
        match stream.shutdown().await {
            Ok(_) => {}
            Err(e) => println!("shutdown error: {}", e),
        };
        for c in commands {
            let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
            let _ = stream.write(c.as_bytes()).await;
            match stream.shutdown().await {
                Ok(_) => {}
                Err(e) => println!("shutdown error: {}", e),
            };
        }
    });
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = Vec::new();
        match socket.read_to_end(&mut buffer).await {
            Ok(_) => {
                println!("{:?}", std::str::from_utf8(&buffer));
                let mut i = 0;
                let buffer_len = buffer.len();
                let mut command_arg = Vec::new();
                'parse: while i < buffer_len {
                    let buf = buffer[i];
                    if buf != b'*' {
                        i += 1;
                        continue;
                    }
                    let start = i;
                    let mut arg_buffer_size = 0usize;
                    let mut args = Vec::new();
                    let mut args_len = 0usize;
                    loop {
                        i += 1;
                        if i >= buffer_len {
                            println!("alone *");
                            break 'parse;
                        }
                        let buf = buffer[i];
                        if buf == b'\r' {
                            arg_buffer_size += i - start + 2;
                            i += 2;
                            break;
                        }
                        match buf.checked_sub(b'0') {
                            Some(n) => {
                                args_len = args_len * 10 + usize::from(n);
                            }
                            None => {
                                println!("error args len")
                            }
                        };
                    }
                    for _ in 0..args_len {
                        if i >= buffer_len {
                            println!("args too much");
                            break 'parse;
                        }
                        let buf = buffer[i];
                        // print!("{:?}", std::str::from_utf8(&[buf]));
                        if buf != b'$' {
                            println!("error arg start");
                            continue 'parse;
                        }
                        let mut str_len = 0usize;
                        let arg_start = i;
                        loop {
                            i += 1;
                            if i >= buffer_len {
                                println!("alone $");
                                break 'parse;
                            }
                            let buf = buffer[i];
                            if buf == b'\r' {
                                arg_buffer_size += i - arg_start + 2;
                                i += 2;
                                break;
                            }
                            match buf.checked_sub(b'0') {
                                Some(n) => {
                                    str_len = str_len * 10 + usize::from(n);
                                }
                                None => {
                                    println!("error arg len")
                                }
                            };
                        }
                        let arg_end = i + str_len;
                        if arg_end >= buffer_len {
                            println!("arg len too much");
                            continue 'parse;
                        }
                        let str_buf = &buffer[i..i + str_len];
                        match std::str::from_utf8(str_buf) {
                            Ok(str) => args.push(str),
                            Err(e) => {
                                println!("{}", e)
                            }
                        };
                        arg_buffer_size += str_len + 2;
                        i = i + str_len + 2;
                    }

                    if args.len() != args_len {
                        println!("args len not the real len");
                        continue 'parse;
                    } else {
                        if i - start != arg_buffer_size {
                            println!("error buffer end");
                        } else {
                            command_arg.push(args);
                        }
                    }
                }
                println!("{:?}", command_arg);
                if !command_arg.is_empty() {
                    for args in command_arg {
                        match parse(args) {
                            Ok(command) => {
                                println!("{}", command.execute());
                            }
                            Err(e) => {
                                println!("{:#?}", e)
                            }
                        };
                    }
                } else {
                    println!("vec is empty");
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
