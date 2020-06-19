use iced::{Command, button, Button, Column, Element, Application, Text, Settings};
use nfd::Response;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn main() {
    TransferrousGui::run(Settings::default())
}

#[derive(Debug, Clone)]
enum LogKind {
    ChoseFile{ file_name: std::ffi::OsString }
}
#[derive(Debug)]
struct LogMessage {
    kind: LogKind,
    timestamp: SystemTime
}

#[derive(Debug)]
struct TransferrousLog {
    messages: Vec<LogMessage>
}

impl Default for TransferrousLog {
    fn default() -> Self {
        Self {
            messages: vec!()
        }
    }
}

#[derive(Default)]
struct TransferrousGui { 
    button_open: button::State,
    log: TransferrousLog,
}

#[derive(Debug, Clone)]
enum Message {
    SendFile,
    FileChosen(Option<PathBuf>),
}

impl Application for TransferrousGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Transferrous")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        println!("\n\nMESSAGE RECEIVED:\n\n{:#?}", message);
        
        let mut messages: Vec<Command<Message>> = vec!();

        match message {
            Message::SendFile => {
                messages.push(Command::perform(send_file(), Message::FileChosen))
            },
            Message::FileChosen(path) => {
                match path {
                    Some(pathbuf) => {
                        let log_kind = LogKind::ChoseFile {
                            file_name: pathbuf.as_os_str().to_owned()
                        };
                        create_log_message(self, log_kind);

                        // TODO
                        println!("File chosen: {}", pathbuf.to_str().unwrap());
                    }
                    None => ()
                };
            }
        }

        Command::batch(messages)
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(Button::new(
                &mut self.button_open, 
                Text::new("Open"))
                .on_press(Message::SendFile))
            .into()
    }
}

fn create_log_message(gui: &mut TransferrousGui, kind: LogKind) {
    gui.log.messages.push(LogMessage{
        kind: kind,
        timestamp: SystemTime::now()
    });
}

async fn send_file() -> Option<PathBuf> {
    match choose_file().await {
        Ok(pathbuf) => Some(pathbuf),
        Err(_) => None
    }
}

async fn choose_file() -> Result<PathBuf, io::Error> {
    let result:nfd::Response = match async {
        return nfd::open_file_dialog(None, None)
    }.await {
        Ok(result) => result,
        Err(e) => return Err(io::Error::new(
            io::ErrorKind::InvalidData, 
            "Unable to unwrap data from new file dialog"
        )),
    };

    let file_string: String = match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(_) => {
            return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput, 
                "Multiple files returned when one was expected"
            )
            )
        }
        Response::Cancel => {
            return Err(
            io::Error::new(
                io::ErrorKind::Interrupted, 
                "User cancelled file open"
            )
            )
        }
    };

    let mut result: PathBuf = PathBuf::new();
    result.push(Path::new(&file_string));

    if result.exists() {
        return Ok(result)
    } else {
        return Err(
        io::Error::new(
            io::ErrorKind::NotFound, 
            "File does not exist"
        )
        )
    }
}      
