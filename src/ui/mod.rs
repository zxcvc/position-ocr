use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::experimental::*;
use fltk::group::*;
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
    pub comfir_button: Button,
    pub group: Group,
    pub dir_input: Input,
    pub select_btn: Button,
}

impl UserInterface {
    pub fn make_window() -> Self {
        let mut main_window = Window::new(2256, 354, 1069, 434, None);
        main_window.set_label(r#"img to json"#);
        main_window.set_type(WindowType::Double);
        main_window.make_resizable(true);
        main_window.set_color(Color::by_index(7));
        main_window.set_frame(FrameType::UpBox);
        main_window.size_range(0, 0, 9999, 99999);
        let mut comfir_button = Button::new(25, 350, 225, 55, None);
        comfir_button.set_label(r#"识别"#);
        let mut group = Group::new(10, 65, 369, 165, None);
        let mut dir_input = Input::new(21, 131, 265, 34, None);
        dir_input.set_label(r#"图片目录"#);
        dir_input.set_align(unsafe { std::mem::transmute(5) });
        dir_input.set_label_font(Font::by_index(4));
        let mut select_btn = Button::new(285, 135, 50, 29, None);
        select_btn.set_label(r#"..."#);
        group.end();
        main_window.end();
        main_window.show();
        Self {
            main_window,
            comfir_button,
            group,
            dir_input,
            select_btn,
        }
    }
}
