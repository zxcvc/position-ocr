
use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::group::experimental::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;

#[derive(Debug, Clone)]
pub struct UserInterface {
    pub main_window: Window,
    pub button: Button,
    pub input: Input,
}

impl UserInterface {
    pub fn make_window() -> Self {
	let mut main_window = Window::new(637, 302, 704, 434, None);
	main_window.set_label(r#"img to json"#);
	main_window.set_type(WindowType::Double);
	main_window.make_resizable(true);
	main_window.set_color(Color::by_index(7));
	let mut button = Button::new(245, 315, 225, 55, None);
	button.set_label(r#"click me"#);
	main_window.resizable(&button);
	let mut input = Input::new(285, 176, 145, 24, None);
	input.set_label_type(LabelType::None);
	main_window.end();
	main_window.show();
	Self {
	    main_window,
	    button,
	    input,
	}
    }
}


