use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::event::{Event, SharedEventsQueue};

pub struct FutureContext {
    pub processed_events: u32,
    pub state: ExecState,
}

#[derive(Debug, PartialEq)]
pub enum ExecState {
    RunOnce,
    Waiting,
}

pub struct FrameFuture;
impl Unpin for FrameFuture {}

impl Future for FrameFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let context: &mut FutureContext = unsafe { std::mem::transmute(context) };

        if context.state == ExecState::RunOnce {
            context.state = ExecState::Waiting;
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct TextureLoadingFuture {
    pub texture: std::rc::Rc<std::cell::RefCell<Option<crate::drawing::Texture2D>>>,
}
impl Unpin for TextureLoadingFuture {}

impl Future for TextureLoadingFuture {
    type Output = crate::drawing::Texture2D;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let context: &mut FutureContext = unsafe { std::mem::transmute(context) };

        if context.state == ExecState::Waiting {
            Poll::Pending
        } else if let Some(texture) = self.texture.borrow_mut().take() {
            context.state = ExecState::Waiting;
            Poll::Ready(texture)
        } else {
            Poll::Pending
        }
    }
}

pub struct EventFuture {
    events: SharedEventsQueue,
}
impl EventFuture {
    pub fn new(events: SharedEventsQueue) -> EventFuture {
        EventFuture { events }
    }
}
impl Unpin for EventFuture {}

impl Future for EventFuture {
    type Output = Option<Event>;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let context: &mut FutureContext = unsafe { std::mem::transmute(context) };

        if context.processed_events < self.events.borrow().len() as u32 {
            let unprocessed_event = context.processed_events;
            context.processed_events += 1;

            let event = self.events.borrow()[unprocessed_event as usize].clone();
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

pub fn resume(future: &mut Pin<Box<dyn Future<Output = ()>>>, context: &mut FutureContext) {
    context.state = ExecState::RunOnce;

    let futures_context_ref: &mut _ = unsafe { std::mem::transmute(context) };
    let _ = future.as_mut().poll(futures_context_ref);
}
