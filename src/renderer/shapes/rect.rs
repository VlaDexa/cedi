use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    points: [Point; 2],
}

macro_rules! swap {
    ($first:expr, $second:expr) => {
        ($first, $second) = ($second, $first)
    };
}

impl Rect {
    pub const fn new(mut first: Point, mut second: Point) -> Self {
        if first.x > second.x {
            swap!(first.x, second.x);
        }
        if first.y > second.y {
            swap!(first.y, second.y);
        }
        let points = [first, second];
        Self { points }
    }
}

impl From<Rect> for sdl2::rect::Rect {
    fn from(rect: Rect) -> Self {
        let [top_left, down_right] = rect.points;
        debug_assert!(top_left.x <= down_right.x);
        debug_assert!(top_left.y <= down_right.y);
        let x: i32 = top_left.x;
        let y: i32 = top_left.y;
        let width = down_right.x - top_left.x;
        let height = down_right.y - top_left.y;
        debug_assert!(width.is_positive());
        debug_assert!(height.is_positive());
        #[allow(clippy::cast_sign_loss)]
        Self::new(x, y, width as u32, height as u32)
    }
}
