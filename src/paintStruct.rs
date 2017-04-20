// pulled from https://github.com/redox-os/orbtk/blob/master/src/widgets/image.rs
extern crate orbtk;
extern crate orbclient;
extern crate orbimage;
use std::rc::Rc;
//use std::borrow::BorrowMut;
use orbtk::{Color, Window, Image, Rect, Point, Renderer, Event};

use orbtk::traits::{Click, Place, EventFilter};
use orbtk::widgets::Widget;


use std::cell::{Cell, RefCell};
use std::path::Path;
use std::sync::Arc;

//Are we using this or canvas? I don't understand
// I think we need to save the tool in paintcanvas, not just main right?



pub struct PaintCanvas {
    pub rect: Cell<Rect>,
    pub image: RefCell<orbimage::Image>,
    click_callback: RefCell<Option<Arc<Fn(&PaintCanvas, Point)>>>,
    click_pos: Rc<RefCell<Option<Point>>>,
    pub tool : Rc<RefCell<Tool>>
}

impl PaintCanvas {
    pub fn new(width: u32, height: u32) -> Arc<Self> {

        Self::from_image(orbimage::Image::new(width, height))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(orbimage::Image::from_color(width, height, color))
    }

    pub fn from_image(image: orbimage::Image) -> Arc<Self> {
        Arc::new(PaintCanvas {
            rect: Cell::new(Rect::new(0, 0, image.width(), image.height())),
            image: RefCell::new(image),
            click_callback: RefCell::new(None),
            click_pos: Rc::new(RefCell::new(None)),
            tool: Rc::new(RefCell::new(Tool::Pen))
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Arc<Self>, String> {
        Ok(Self::from_image(orbimage::Image::from_path(path)?))
    }
}

impl Click for PaintCanvas {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for PaintCanvas {}

impl Widget for PaintCanvas {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        let image = self.image.borrow();
        renderer.image(rect.x, rect.y, image.width(), image.height(), image.data());
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        let t_0 = self.tool.clone();
        let mut t = t_0.borrow_mut();
        match event {
            Event::Mouse { point, left_button, .. } => {
                let click = self.click_pos.clone();
                let mut prev_opt = click.borrow_mut();
                if left_button {
                    if *t == Tool::Pen || *t == Tool::Erase {
                        println!("{:?}", *t);
                        if let Some(prev_position) = *prev_opt {
                            let mut image = self.image.borrow_mut();
                            let color = if *t == Tool::Pen {0} else {255};
                            image.line(prev_position.x, prev_position.y, point.x,
                            point.y, orbtk::Color::rgb(color, color, color));
                        }
                        *prev_opt = Some(point);
                    }
                /*
                let rect = self.rect.get();
                if rect.contains(point) && left_button {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                    *redraw = true;
                }*/
                }
                else{
                    *prev_opt = None;
                }
            }
            Event::Text { c } => {
                if c == 'p' {*t = Tool::Pen; print!("Changing to pen tool");}
                else if c == 'e' {*t = Tool::Erase; print!("Changing to erase tool");}
                else if c == 'f' {*t = Tool::Fill; print!("Changing to fill tool");}
                else if c == 'l' {*t = Tool::Line; print!("Changing to line tool");}
                else{print!("Unknown tool {} ", c);}

            }
            _ => {}//print!("Something else!");}
        }

        //focused

        true
    }
}

pub fn main() {
    //let mut tool : Tool = Tool::Pen;
    let mut window = Window::new(Rect::new(100, 100, 420, 420), "Canvas");
    let mut tools = Window::new(Rect::new(100, 100, 105, 420), "Tools");


    let canvas = PaintCanvas::from_color(400, 400, Color::rgb(255, 255, 255));
    /*canvas.position(10, 10)
        .on_click(move |canvas: &PaintCanvas, point: Point| {
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
        });*/


    let tools = PaintCanvas::from_color(25, 420, Color::rgb(255, 255, 255));
    tools.position(15, 15);
    window.add(&tools);
    window.add(&canvas);
    window.exec();

}

#[derive(PartialEq, Clone, Debug)]
enum Tool{
    Pen,
    Erase,
    Fill,
    Line,
    Rectangle,
    Circle,
}
