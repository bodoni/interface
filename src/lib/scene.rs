use Object;
use display::Display;
use frame::Frame;
use outcome::Result;

/// A scene.
pub struct Scene {
    objects: Vec<Box<Object>>,
}

impl Scene {
    /// Create a scene.
    #[inline]
    pub fn new(_: &Display) -> Result<Scene> {
        Ok(Scene { objects: vec![] })
    }

    /// Append an object to the scene.
    #[inline]
    pub fn append<T: 'static + Object>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Object for Scene {
    fn render(&self, frame: &mut Frame) -> Result<()> {
        for object in self.objects.iter() {
            try!(object.render(frame));
        }
        Ok(())
    }
}
