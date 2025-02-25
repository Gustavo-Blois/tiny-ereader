use iced::widget::{button, row, container, Row, Column, column};
use iced::{Alignment, Element};
use std::process::{Command, Stdio};

#[derive(Default, Clone)]
enum Mode {
    #[default]
    Menu,
    Books,
    Vns,
    
}

#[derive(Debug, Clone)]
enum Message {
    Menu,
    Books,
    Vns,
    OpenBook(String),
    OpenVn(String),
}

fn main() -> iced::Result {
    iced::run("Tiny E-reader", Mode::update, Mode::view)
}

impl Mode {
    fn update(&mut self, message: Message) {
        match message {
            Message::Menu => *self = Mode::Menu,
            Message::Books => *self = Mode::Books,
            Message::Vns => *self = Mode::Vns,
            Message::OpenBook(s) => {open_book(s);}
            Message::OpenVn(s) => {open_vn(s);}
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Mode::Menu => self.view_menu(),
            Mode::Books => self.view_books(),
            Mode::Vns => self.view_vns(),
        }
    }

    fn view_menu(&self) -> Element<Message> {
        row![button("Books").on_press(Message::Books),button("Visual Novels").on_press(Message::Vns)]
            .padding(20)
            .align_y(Alignment::Center)
            .into()
    }

    fn view_books(&self) -> Element<Message> {

        let output = Command::new("ls")
            .arg("./books")
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output())
            .expect("Failed to retrieve books");
    
        let books_str = String::from_utf8(output.stdout)
            .unwrap()
            .into_boxed_str();
        let books: &'static str = Box::leak(books_str);
        let book_list: Vec<&str> = books.lines().collect();
    
        let mut columns = Column::new().spacing(10);
        
        for &book in &book_list {
            columns = columns.push(
                button(book)
                    .on_press(Message::OpenBook(book.to_string()))
            );
        }
        
        columns = columns.push(button("Menu").on_press(Message::Menu));
        columns.into()
    }

    fn view_vns(&self) -> Element<Message> {
        let output = Command::new("ls")
            .arg("./vns")
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output())
            .expect("Failed to retrieve vns");
        let vns_str = String::from_utf8(output.stdout)
            .unwrap()
            .into_boxed_str();
        let vns: &'static str = Box::leak(vns_str);
        let vn_list: Vec<&str> = vns.lines().collect();

        let mut rows = Row::new().spacing(10);

        for &vn in &vn_list {
            rows = rows.push(button(vn).on_press(Message::OpenVn(vn.to_string())));
        }
        rows = rows.push(button("Menu").on_press(Message::Menu));
        rows.into()

    }
    
}

fn open_vn(s: String) {
    let path = format!("./vns/{}",s);
    let catch_vn = Command::new("ls").arg(path).stdout(Stdio::piped()).spawn().expect("Failed to visualize vn");
    let catch_out = catch_vn.stdout.expect("Failed to open catch_vn stdout");
    let filter_sh = Command::new("grep").arg(".sh").stdin(Stdio::from(catch_out)).stdout(Stdio::piped()).spawn().and_then(|child| child.wait_with_output()).expect("Failed to filter vn .sh");
    let vn_sh_path = String::from_utf8(filter_sh.stdout).unwrap();
    let vn_sh_path_clean = vn_sh_path.replace("\n","");
    Command::new("sh").arg(format!("./vns/{}/{}",s,vn_sh_path_clean)).spawn().expect("Failed to open vn");
}

fn open_book(s: String){
    let path = format!("./books/{}",s);
    Command::new("xdg-open").arg(path).spawn().expect("Failed to open book");
}
