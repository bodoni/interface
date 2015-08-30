use Result;

pub trait Object {
    fn render(&self) -> Result<()>;
}
