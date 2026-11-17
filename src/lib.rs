use std::path::Iter;

#[derive(Debug)]
struct Thing {
    field: f64,
}

#[derive(Debug)]
struct TwoThings<'a> {
    a: &'a Thing,
    b: &'a Thing,
}

#[derive(Debug)]
struct SeveralThings {
    things: Vec<Thing>,
}

impl SeveralThings {
    // fn pairs(&self) -> SeveralThingsIterator {
    //     SeveralThingsIterator::new(self)
    // }

    fn pairs(&self) -> impl Iterator<Item = TwoThings> {
        self.things
            .iter()
            .zip(self.things.iter().cycle().skip(1))
            .map(|pair| TwoThings {
                a: pair.0,
                b: pair.1,
            })
    }
}

struct SeveralThingsIterator<'a> {
    things: &'a SeveralThings,
    index: usize,
}

impl<'a> SeveralThingsIterator<'a> {
    fn new(things: &'a SeveralThings) -> Self {
        Self { things, index: 0 }
    }
}

impl<'a> Iterator for SeveralThingsIterator<'a> {
    type Item = TwoThings<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let n_items = self.things.things.len();
        if index >= n_items {
            return None;
        }
        let a = &self.things.things[index];
        let b = &self.things.things[(index + 1) % n_items];
        self.index += 1;
        Some(TwoThings { a, b })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mytest() {
        let things = SeveralThings {
            things: vec![
                Thing { field: 0.0 },
                Thing { field: 1.0 },
                Thing { field: 2.0 },
                Thing { field: 3.0 },
                Thing { field: 4.0 },
                Thing { field: 5.0 },
                Thing { field: 6.0 },
                Thing { field: 7.0 },
            ],
        };
        for pair in things.pairs() {
            println!("{pair:?}");
        }
    }
}
