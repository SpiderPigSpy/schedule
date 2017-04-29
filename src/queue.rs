use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering};

use time::*;

pub struct Queue<T> {
    heap: BinaryHeap<Item<T>>
}

#[derive(PartialEq, Eq, Debug)]
pub enum NextItem<T> {
    After(DesiredTime),
    Now(T),
    Never
}

struct Item<T> {
    desired_time: DesiredTime,
    data: T
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            heap: BinaryHeap::new()
        }
    }

    pub fn put(&mut self, data: T, time: DesiredTime) {
        self.heap.push(Item::new(time, data));
    }

    pub fn next(&mut self) -> NextItem<T> {
        match self.heap.pop() {
            Some(Item{desired_time, data}) => {
                if desired_time.is_ready() {
                    NextItem::Now(data)
                } else {
                    self.heap.push(Item::new(desired_time, data));
                    NextItem::After(desired_time)
                }
            },
            None => NextItem::Never
        }
    }
}

impl<T> Item<T> {
    fn new(desired_time: DesiredTime, data: T) -> Self {
        Item {
            desired_time: desired_time,
            data: data
        }
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.desired_time.cmp(&self.desired_time)
    }
}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            other.desired_time.cmp(&self.desired_time)
        )
    }
}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.desired_time == other.desired_time
    }
}

impl<T> Eq for Item<T> {

}

#[cfg(test)]
mod tests {
    use ::chrono::Duration;
    use ::chrono::offset::local::Local;
    use super::*;

    #[test]
    fn returns_never_when_queue_is_empty() {
        //given
        let mut queue = Queue::<()>::new();
        //when
        let next = queue.next();
        //then
        assert_eq!(NextItem::Never, next);
    }

    #[test]
    fn returns_data_when_desired_time_was_now() {
        //given
        let mut queue = Queue::<()>::new();
        queue.put((), DesiredTime::now());
        //when
        let next = queue.next();
        //then
        assert_eq!(NextItem::Now(()), next);
    }

    #[test]
    fn returns_data_with_smalles_desired_time() {
        //given
        let now = Local::now().naive_local();
        let mut queue = Queue::<i32>::new();
        queue.put(0, now.into());
        queue.put(1, now.checked_add_signed(Duration::milliseconds(1)).unwrap().into());
        //when
        let next = queue.next();
        //then
        assert_eq!(NextItem::Now(0), next);
    }
}
