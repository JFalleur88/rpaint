extern crate orbtk;
use std::thread;
use orbtk::{Color, Window, Image, Rect, Point, Renderer, Event};

use orbtk::traits::{Click, Place, EventFilter};
use orbtk::widgets::Widget;
use std::rc::Rc;
use std::cell::RefCell;

//TODO:
//
//-can lift pen, but is so laggy that can only draw a continuous line
//if you draw very slowly (bc it does check click position every frame, i think,
//but redox is so laggy that that means it checks the click position like 2
//frames per second)
//-eraser tool has same issue

//-fix toolbar (does nothing right now)

//impl Widget for Paint {
//    fn rect(&self) -> &Cell<Rect> {
//        &self.rect
//    }
//
//    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
//        let rect = self.rect.get();
//        let image = self.image.borrow();
//        renderer.image(rect.x, rect.y, image.width(), image.height(), image.data());
//    }
//
//    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
//        match event {
//            Event::Mouse { point, left_button, .. } => {
//                let rect = self.rect.get();
//                if rect.contains(point) && left_button {
//                    let click_point: Point = point - rect.point();
//                    self.emit_click(click_point);
//                    *redraw = true;
//                }
//            }
//            Event::Text { c } => {
//
//            }
//            _ => (),
//        }
//
//        focused
//    }
//}

/*pub struct Paint {
    //tool : u32,
    //mouseDown : bool,
    // contain if mouse is clicked or held down
    // contain currently selected text
}

impl Paint {
    pub fn new() -> Self {

    }


    // make new window



    fn erase() {

    }

    fn tool_setup() {

    }

}


fn main() {
    setup();



}*/

pub fn main() {
    let mut tool = "P";
    let mut window = Window::new(Rect::new(100, 100, 420, 420), "Canvas");
    let mut tools = Window::new(Rect::new(100, 100, 105, 420), "Tools");

    let click_pos: Rc<RefCell<Option<Point>>>= Rc::new(RefCell::new(None));
    let canvas = Image::from_color(400, 400, Color::rgb(255, 255, 255));
    canvas.position(10, 10)
        .on_click(move |canvas: &Image, point: Point| {
            let click = click_pos.clone();
            //self.tool = Event.Text;
            {


                let mut prev_opt = click.borrow_mut();

                if let Some(prev_position) = *prev_opt {
                    let mut image = canvas.image.borrow_mut();
                    if (prev_position.x - point.x).abs() <= 1 &&
                    (prev_position.y - point.y).abs() <= 1 {
                        if tool == "P" {
                            image.line(prev_position.x, prev_position.y, point.x,
                            point.y, orbtk::Color::rgb(0, 0, 0));

                        } else if tool == "E" {
                            image.line(prev_position.x, prev_position.y, point.x,
                            point.y, orbtk::Color::rgb(255, 255, 255));
                        } else {
                            print!("No tool selected")
                        }
                    }
                    *prev_opt = Some(point);
                } else {
                    *prev_opt = Some(point);
                }
            }
        });


    let tools = Image::from_color(25, 420, Color::rgb(255, 255, 255));
    tools.position(15, 15);
    window.add(&tools);
    window.add(&canvas);
    window.exec();

}
