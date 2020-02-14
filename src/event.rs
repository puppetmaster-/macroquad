use miniquad as mq;

use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown(mq::KeyCode),
    KeyUp(mq::KeyCode),
}

pub type SharedEventsQueue = Rc<RefCell<Vec<Event>>>;
