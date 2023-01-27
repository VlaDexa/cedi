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

    /// Splits given rectangle into an Iterator of n equal rectangles split horizontally
    pub fn split_horizontal(self, n: u32) -> impl Iterator<Item = Self> {
        let [top_left, down_right] = self.points;
        let height = down_right.y - top_left.y;
        let single_height = height / n;
        (0..n).map(move |i| {
            Self::new(
                Point::new(top_left.x, top_left.y + single_height * i),
                Point::new(down_right.x, down_right.y - single_height * (n - i - 1)),
            )
        })
    }
}

impl From<Rect> for sdl2::rect::Rect {
    fn from(rect: Rect) -> Self {
        let [top_left, down_right] = rect.points;
        let x = i32::try_from(top_left.x);
        let y = i32::try_from(top_left.y);
        debug_assert!(top_left.x <= down_right.x);
        debug_assert!(top_left.y <= down_right.y);
        let width = down_right.x - top_left.x;
        let height = down_right.y - top_left.y;
        debug_assert!(x.is_ok());
        debug_assert!(y.is_ok());
        Self::new(
            x.expect("Rect coordinates are over i32::MAX"),
            y.expect("Rect coordinates are over i32::MAX"),
            width,
            height,
        )
    }
}
