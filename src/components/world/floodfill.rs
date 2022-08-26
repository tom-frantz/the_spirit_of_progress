use crate::components::world::latlon::{LatLonPoint, ValuePoint, WorldPoint};
use bevy::utils::HashSet;
use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct FloodFillPriority<T>
where
    T: Clone + Debug,
{
    pub priority: f32,
    pub index: usize,
    pub point: ValuePoint<T>,
}

impl<T> FloodFillPriority<T>
where
    T: Clone + Debug,
{
    pub fn new(priority: f32, point: ValuePoint<T>, index: usize) -> Self {
        FloodFillPriority {
            priority,
            point,
            index,
        }
    }
}

impl<T> Eq for FloodFillPriority<T> where T: Clone + Debug {}

impl<T> PartialEq<Self> for FloodFillPriority<T>
where
    T: Clone + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.index == other.index
    }
}

impl<T> PartialOrd<Self> for FloodFillPriority<T>
where
    T: Clone + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.priority == other.priority {
            if self.index == other.index {
                Some(Ordering::Equal)
            } else if self.index > other.index {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else if self.priority > other.priority {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<T> Ord for FloodFillPriority<T>
where
    T: Clone + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.priority == other.priority {
            if self.index == other.index {
                Ordering::Equal
            } else if self.index > other.index {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if self.priority > other.priority {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

pub struct FloodFill<T>
where
    T: Debug + Clone,
{
    visited: FxHashSet<LatLonPoint>,
    next_index: usize,
    queue: BTreeSet<FloodFillPriority<T>>,
}

impl<T> FloodFill<T>
where
    T: Clone + Debug,
{
    pub fn new() -> Self {
        FloodFill {
            visited: FxHashSet::default(),
            next_index: 0,
            queue: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, priority: f32, value: ValuePoint<T>) {
        if self.visited.contains(&value.point) {
            return;
        }

        self.queue.insert(FloodFillPriority {
            priority,
            index: self.next_index,
            point: value,
        });

        self.next_index += 1;
    }

    pub fn next(&mut self) -> Option<FloodFillPriority<T>> {
        println!("Visited Count: {}", self.visited.len());
        loop {
            let next = self.queue.pop_first();
            match next {
                None => return None,
                Some(fill_priority) => {
                    if (fill_priority.point.lat() == 0. && fill_priority.point.lon() == 0.) {
                        println!(
                            "{:?} Contains? {}",
                            fill_priority,
                            self.visited.contains(&fill_priority.point.point)
                        );
                    }
                    if !self.visited.contains(&fill_priority.point.point) {
                        // Does not contain
                        println!("insert {:?}", fill_priority.point.point);
                        self.visited.insert(fill_priority.point.point);
                        return Some(fill_priority);
                    } else {
                        // Does contain, skip since it's already popped lmao.
                        continue;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone)]
    struct MyStruct {
        value: String,
    }

    impl MyStruct {
        pub fn new(value: String) -> Self {
            MyStruct { value }
        }
    }

    use crate::components::world::floodfill::FloodFill;
    use crate::components::world::latlon::{LatLonPoint, ValuePoint};

    #[test]
    fn testing() {
        let mut flood: FloodFill<MyStruct> = FloodFill::new();

        flood.insert(
            2.0,
            ValuePoint::new(
                LatLonPoint::new(0.0, 0.0),
                MyStruct::new(String::from("General Kenobi!")),
            ),
        );
        flood.insert(
            1.0,
            ValuePoint::new(
                LatLonPoint::new(0.0, 0.0),
                MyStruct::new(String::from("Hello There")),
            ),
        );

        assert_eq!(
            flood.next().unwrap().point.value.value,
            String::from("Hello There")
        );
        assert_eq!(
            flood.next().unwrap().point.value.value,
            String::from("General Kenobi!")
        );
    }
}
