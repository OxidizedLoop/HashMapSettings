use crate::account::Incrementable;

impl Incrementable for i32 {
    fn increment_mut(&mut self) {
        *self += 1; //todo!(incrementable)
    }

    fn increment(&self) -> Self {
        self + 1 //todo!(incrementable)
    }
}
impl Incrementable for String {
    fn increment_mut(&mut self) {
        *self += "(1)"; //todo!(incrementable)
    }

    fn increment(&self) -> Self {
        self.to_owned() + "(1)" //todo!(incrementable)
    }
}
