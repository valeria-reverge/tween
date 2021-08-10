extern crate simple;
use simple::{Rect, Window};
use vector2d::Vector2D;
use std::cell::RefCell;

static SCREEN_WIDTH: u16 = 1280;
static SCREEN_HEIGHT: u16 = 720;

trait Tweenable
{
    fn get_pos(&self) -> Vector2D<f32>;
    fn set_pos(&mut self, pos: Vector2D<f32>);
}

enum InterpolationType
{
    Linear,
}

struct Tween<'a>
{
    how: InterpolationType,
    t: f32,
    p0: Vector2D<f32>,
    p1: Vector2D<f32>,
    tweenable: &'a RefCell<dyn Tweenable>,
}

impl <'a> Tween<'a>
{
    fn new(tweenable: &'a RefCell<dyn Tweenable>, p1: Vector2D<f32>, how: InterpolationType) -> Tween<'a> 
    {
        Tween
        {
            how: how,
            t: 0.0f32,
            p0: tweenable.borrow().get_pos(),
            p1: p1,
            tweenable: tweenable,
        }
    }
    fn update(&mut self, delta_t: f32)
    {
        self.t = (self.t + delta_t).min(1.0f32);
        match self.how
        {
            InterpolationType::Linear => self.tweenable.borrow_mut().set_pos(self.p0 * (1.0f32 - self.t) + self.p1 * self.t)
        }
    }
}



struct Square
{
    pos: Vector2D<f32>,
}

impl Square
{
    fn new(x: f32, y: f32) -> Self
    {
        Square
        {
            pos: Vector2D::new(x, y),
        }
    }
    fn draw(&self, app: &mut Window)
    {
        app.fill_rect(Rect::new(self.pos.x as i32 - 32, self.pos.y as i32 - 32, 64, 64));
    }
}

impl Tweenable for Square
{
    fn get_pos(&self) -> Vector2D<f32>
    {
        self.pos
    }
    fn set_pos(&mut self, pos: Vector2D<f32> )
    {
        self.pos = pos;
    }
}


fn main()
{
    let mut app = Window::new("Squares", SCREEN_WIDTH, SCREEN_HEIGHT);

    let squares = vec![
        RefCell::new(Square::new(50.0f32, 50.0f32)),
        RefCell::new(Square::new(50.0f32, 150.0f32)),
    ];

    let mut tweens = vec![
        Tween::new(&squares[0], Vector2D::<f32>::new(500.0f32, 150.0f32), InterpolationType::Linear),
        Tween::new(&squares[1], Vector2D::<f32>::new(500.0f32, 50.0f32), InterpolationType::Linear),
    ];

    while app.next_frame()
    {
        app.clear();

        for tween in tweens.iter_mut()
        {
            tween.update(0.01f32);
        }

        for square in squares.iter()
        {
            square.borrow().draw(&mut app);
        }
    }
}
