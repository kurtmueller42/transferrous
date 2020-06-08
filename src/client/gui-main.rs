use iced::{slider, Command, button, Button, Column, Element, ProgressBar, Sandbox, Text, Settings, Slider};
use nfd::Response;
use std::io;
use std::path::{Path, PathBuf};

pub fn main() {
    Progress::run(Settings::default())
}

#[derive(Default)]
struct Progress {
    value: f32,
    progress_bar_slider: slider::State,
    button_open: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f32),
    OpenFile,
    OpenedFile(Option<String>),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
            Message::OpenFile => {
                Command::perform(send_file(), Message::OpenedFile);
            },
            Message::OpenedFile(foo) => {
                // TODO
                self.value = 100.0 - self.value
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(ProgressBar::new(0.0..=100.0, self.value))
            .push(Slider::new(
                &mut self.progress_bar_slider,
                0.0..=100.0,
                self.value,
                Message::SliderChanged,
            ))
            .push(Button::new(
                &mut self.button_open, 
                Text::new("Open"))
                .on_press(Message::OpenFile))
            .into()
    }
}

async fn send_file() -> Option<String> {
    let res = choose_file().await;
    // TODO
    return None;
}

async fn choose_file() -> Result<PathBuf, io::Error> {

    let result:nfd::Response = match async {
        return nfd::open_file_dialog(Some("json"), None)
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
