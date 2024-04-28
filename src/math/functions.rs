pub struct FunctionRToR {
    pub f: Box<dyn Fn(f64) -> f64>
}

impl FunctionRToR {
    pub fn new(f: Box<dyn Fn(f64) -> f64>) -> Self {
        FunctionRToR {
            f
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        return (self.f)(x);
    }
}

pub struct FunctionR2ToR {
    pub f: Box<dyn Fn(f64, f64) -> f64>
}

impl FunctionR2ToR {
    pub fn new(f: Box<dyn Fn(f64, f64) -> f64>) -> Self {
        FunctionR2ToR {
            f
        }
    }

    pub fn eval(&self, x: f64, y: f64) -> f64 {
        return (self.f)(x, y);
    }
}
