pub struct Bunch<'d, T> where T: 'd {
    data: &'d [T],
    dose: usize,
}

impl<'d, T> Bunch<'d, T> {
    #[inline]
    pub fn new(data: &'d [T], dose: usize) -> Bunch<'d, T> {
        Bunch {
            data: data,
            dose: dose,
        }
    }
}

impl<'d, T> Iterator for Bunch<'d, T> {
    type Item = &'d [T];

    fn next(&mut self) -> Option<&'d [T]> {
        let (left, need) = (self.data.len(), self.dose);
        if left >= need {
            let some = &self.data[..need];
            self.data = &self.data[need..];
            Some(some)
        } else {
            None
        }
    }
}
