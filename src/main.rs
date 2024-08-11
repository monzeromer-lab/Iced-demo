use iced::font::{Family, Weight};
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, progress_bar, row, scrollable,
    slider, text, text_input, toggler, vertical_rule, vertical_space,
};
use iced::{
    executor, Alignment, Application, Command, Element, Font, Length, Pixels, Renderer, Size, Theme,
};

pub fn main() -> iced::Result {
    Styling::run(iced::Settings {
        window: iced::window::Settings {
            size: Size {
                width: 377.0,
                height: 533.0,
            },
            position: iced::window::Position::Centered,
            ..Default::default()
        },
        default_font: Font {
            family: Family::Fantasy,
            weight: Weight::Normal,
            ..Default::default()
        },
        default_text_size: Pixels(14.0),
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Default)]
struct Styling {
    theme: Theme,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
    username: String,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    UserNameChanged(String),
}

impl Application for Styling {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Styling, Command<Self::Message>) {
        (Styling::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Awesome Iced Demo")
    }

    fn update(&mut self, message: Message) -> Command<self::Message> {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::InputChanged(value) => self.input_value = value,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::UserNameChanged(value) => self.username = value,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Length::Fill),
        ]
        .spacing(10);

        let random_text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(5)
            .size(14);

        let another_text_input: text_input::TextInput<'_, Message, Theme, Renderer> =
            text_input("What is Your Name?", &self.username)
                .on_input(Message::UserNameChanged)
                .padding(5)
                .size(14);

        let button = button("Submit").padding(5);

        let slider = slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = progress_bar(0.0..=100.0, self.slider_value);

        let scrollable = scrollable(column![
            "Scroll me!",
            vertical_space().height(800),
            "You did it!"
        ])
        .width(Length::Fill)
        .height(100);

        let checkbox =
            checkbox("Check me!", self.checkbox_value).on_toggle(Message::CheckboxToggled);

        let toggler = toggler(
            String::from("Toggle me!"),
            self.toggler_value,
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10);

        let some_other_content = row![another_text_input];

        let content = column![
            choose_theme,
            horizontal_rule(38),
            row![random_text_input, button]
                .spacing(10)
                .align_items(Alignment::Center),
            slider,
            progress_bar,
            row![
                scrollable,
                vertical_rule(38),
                column![checkbox, toggler].spacing(20)
            ]
            .spacing(10)
            .height(100)
            .align_items(Alignment::Center),
            row![some_other_content]
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
