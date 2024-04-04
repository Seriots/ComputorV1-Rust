use std::fmt::Display;


pub struct Sqrt {
    pub whole_part: Vec<i32>,
    pub sqrt_part: Option<u32>,
}

impl Sqrt {
    pub fn new(whole_part: Vec<i32>, sqrt_part: Option<u32>) -> Self {
        Sqrt { whole_part, sqrt_part }
    }

    pub fn more_sqrt(&mut self, n: u32) {
        if let Some(sqrt_v) = self.sqrt_part {
            self.sqrt_part = Some(sqrt_v * n);
        } else {
            self.sqrt_part = Some(n);
        }
    }
}

impl Display for Sqrt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let whole_part = self.whole_part.iter().product::<i32>();
        let sign = if whole_part < 0 { "-" } else { "+" };
        if let Some(sqrt_v) = self.sqrt_part {
            if whole_part.abs() == 1 {
                return write!(f, " {} √{}", sign, sqrt_v)
            } else {
                write!(f, " {} {} * √{}", sign, whole_part.abs(), sqrt_v)
            }
        }
        else {
            write!(f, "{}{}", sign, whole_part.abs())
        }
    }
}
