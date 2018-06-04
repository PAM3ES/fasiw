#[macro_use]
extern crate allegro;
extern crate allegro_font;
extern crate allegro_primitives;
extern crate allegro_sys;

#[macro_use]
extern crate derive_new;

use allegro::*;
use allegro_font::*;
use allegro_primitives::*;
use allegro_sys::*;
const CLICK: i32 = 1;

mod FASIW {
    pub static mut x_mouse_click: f32 = -100.0;
    pub static mut y_mouse_click: f32 = -100.0;
    pub static mut mouse_status: i32 = 0;
    pub static mut number_click: i32 = 0;

    pub fn x_writter(ch1: f32) {
        unsafe {
            x_mouse_click = ch1;
        }
    }
    pub fn y_writter(ch2: f32) {
        unsafe {
            y_mouse_click = ch2;
        }
    }
    pub fn get_x() -> f32 {
        unsafe {
            let x_ret = x_mouse_click;
            x_ret
        }
    }
    pub fn get_y() -> f32 {
        unsafe {
            let y_ret = y_mouse_click;
            y_ret
        }
    }
    pub fn change_mouse_status(changer_number: i32) {
        unsafe {
            mouse_status = changer_number;
        }
    }
    pub fn return_mouse_status() -> i32 {
        unsafe { mouse_status }
    }
    pub fn init() {}
    pub fn click_for_numbers() {
        unsafe {
            number_click = number_click + 1;
        }
    }
    pub fn return_click_mouse() -> i32 {
        unsafe { number_click }
    }
    pub fn set_click_for_numbers(num11: i32) {
        unsafe {
            number_click = num11;
        }
    }
}

trait ActionWith<'a> {
    fn draw(&self);
    fn click(&mut self, i32) -> i32;
    fn ChangeColor(&mut self, u8, u8, u8, u8, &PrimitivesAddon);
    fn click_inside<T>(&self, T) -> bool;
}

#[derive(new)]
struct Button<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    alo: i64,
    rgba: Color,
    q: &'a PrimitivesAddon,
}

impl<'a> ActionWith<'a> for Button<'a> {

	fn click_inside<T>(&self, obj: T) -> bool {
	if FASIW::get_x() > self.x && 
       FASIW::get_x() < self.w && 
       FASIW::get_y() > self.y && 
       FASIW::get_y() < self.h {
       	true
       } else { false }
	}

    fn draw(&self) {
        self.q
            .draw_filled_rectangle(self.x, self.y, self.w, self.h, self.rgba);
    }

    fn click(&mut self, returN: i32) -> i32 {
        if self.click_inside(&self) == true
        {
            1
        } else {
            2
        }
    }

    fn ChangeColor(&mut self, r: u8, g: u8, b: u8, a: u8, q: &PrimitivesAddon) {
        self.rgba = Color::from_rgba(r, g, b, a);
        q.draw_filled_rectangle(self.x, self.y, self.w, self.h, self.rgba);
    }
}

#[derive(new)]
struct CheckBox<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    alo: i64,
    rgba: Color,
    q: &'a PrimitivesAddon,
    #[new(value = "0")]
    checkbox_clicks: i32,
}

impl<'a> ActionWith<'a> for CheckBox<'a> {
	fn click_inside<T>(&self, obj: T) -> bool {
	if FASIW::get_x() > self.x && 
       FASIW::get_x() < self.w && 
       FASIW::get_y() > self.y && 
       FASIW::get_y() < self.h {
       	true
       } else { false }
	}

    fn draw(&self) {
        self.q
            .draw_filled_rectangle(self.x, self.y, self.w, self.h, self.rgba);
    }

    fn click(&mut self, returN: i32) -> i32 {
        unsafe {
            if self.click_inside(&self) == true
            {
                self.checkbox_clicks = self.checkbox_clicks + 1;
                if (self.checkbox_clicks % 2) == 0 {
                    return 1;
                } else {
                    return 2;
                }
            }
            return 0;
        }
    }

    fn ChangeColor(&mut self, r: u8, g: u8, b: u8, a: u8, q: &PrimitivesAddon) {
        self.rgba = Color::from_rgba(r, g, b, a);
        q.draw_filled_rectangle(self.x, self.y, self.w, self.h, self.rgba);
    }
}

#[derive(new)]
struct Texter<'a> {
    core: &'a Core,
    x: f32,
    y: f32,
    rgba: Color,
    text: &'a str,
    size: i32,
}

impl<'a> Texter<'a> {
    fn draw(&mut self, ff: &Font) {
        self.core.draw_text(
            &ff,
            self.rgba,
            self.x,
            self.y,
            FontAlign::Centre,
            &self.text,
        );
    }
}

#[derive(new)]
struct InputText<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    rgba: [Color; 2],
    text: &'a str,
    size: i32,
    border: [f32; 2],
    q: &'a PrimitivesAddon,
    #[new(value = "0")]
    moveindex: usize,
}

impl <'a>ActionWith<'a> for InputText<'a>  {

	fn click_inside<T>(&self, obj: T) -> bool {
	if FASIW::get_x() > self.x && 
       FASIW::get_x() < self.w && 
       FASIW::get_y() > self.y && 
       FASIW::get_y() < self.h {
       	true
       } else { false }
	}

    fn draw(&self) {
        self.q
            .draw_filled_rectangle(self.x, self.y, self.w, self.h, self.rgba[self.moveindex]);
    }

    fn click(&mut self, returN: i32) -> i32 {
        unsafe {
            if self.click_inside(&self) == true
            {
                self.moveindex = 1;
                return 1;
            } else {
                self.moveindex = 0;
                return 0;
            }
        }
    }

    fn ChangeColor(&mut self, r: u8 , g: u8, b: u8, a: u8, q: &PrimitivesAddon) {
        self.rgba[1] = Color::from_rgba(r, g, b, a);
    }
}

allegro_main!
{
  let mut x_mouse: f32 = 0.0;
  let mut y_mouse: f32 = 600.0;
  let mut switcher1: u8 = 0;
  let mut colorsearch: u8 = 255;

    let core = Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();
    let mut nco: i32 = 0;
    let mut  primit1 = PrimitivesAddon::init(&core).unwrap();

    let display = Display::new(&core, 800, 400).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font = Font::new_builtin(&font_addon).unwrap();
    core.install_mouse().unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_mouse_event_source().unwrap());

let mut chb1 = CheckBox{x: 200.0, y: 200.0, w: 250.0,
                        h: 250.0, alo: 25, rgba:
                        Color::from_rgba(10,10,10,255), q: &primit1, checkbox_clicks: 0};
let mut chb2 = CheckBox{x: 300.0, y: 300.0, w: 400.0,
                        h: 400.0, alo: 25, rgba:
                        Color::from_rgba(10,10,10,255), q: &primit1, checkbox_clicks: 0 };
let mut Bu1 = Button{x: 10.0, y: 10.0,
                    w: 150.0, h: 150.0,
                    alo: 25, rgba: Color::from_rgba(10,10,10,255), q: &primit1};
let mut text1 = Texter{core: &core, x: 250.0 ,y: 250.0 ,rgba: Color::from_rgba(255, 255, 255, 255), text: "nigga", size: 15 };

let mut ipt = InputText::new(10.0 , 10.0, 150.0 , 150.0,
 [Color::from_rgba(255, 255, 255, 255), Color::from_rgba(150, 155, 155, 255)], "bed", 15 ,  [15.0,25.0], &primit1);

    let mut redraw = true;
    timer.start();
    'exit: loop
    {

        if redraw && queue.is_empty()
        {

          let buffsst: String = nco.to_string();
            core.clear_to_color(Color::from_rgb_f(255.0,255.0,255.0));
        primit1.draw_filled_rectangle(0.0, 0.0, 2000.0, 2000.0, Color::from_rgba(10, 10, 150, 255));
            core.draw_text(&font,Color::from_rgba(255, 255, 255, 255), 100.0,100.0, FontAlign::Centre, "123");
            text1.draw(&font);
            ipt.draw();
            core.flip_display();
            redraw = false;

        }

        match queue.wait_for_event()
        {
            DisplayClose{..} => break 'exit,
            TimerTick{..} => redraw = true,
            MouseAxes{x, y , ..} => {x_mouse = (x) as f32;  y_mouse = (y) as f32; if ipt.click_inside(&ipt) {println!("0");}},
            MouseButtonUp{x,y, ..} => {FASIW::change_mouse_status(1);  },
            MouseButtonDown{x,y, ..} => {
                FASIW::click_for_numbers();
                FASIW::change_mouse_status(0);
                FASIW::x_writter((x) as f32);
                FASIW::y_writter((y) as f32);

                ipt.click(1);

                },
            MouseLeaveDisplay{..} => {x_mouse = -100.0; y_mouse = 576.0;},
            _ => (),
        }
    }
}
