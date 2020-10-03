use crate::import::*;


/// turn gloo-events into an async stream.
//
pub struct EHandler
{
	receiver: UnboundedReceiver<Event>,

	// Automatically removed from the DOM on drop!
	//
	_listener: EventListener,
}


impl EHandler
{
	/// When prevent_default is true, the event will not trigger the browsers default action.
	/// eg. like checking a checkbox when clicked.
	//
	pub fn new( target: &EventTarget, event: &'static str, prevent_default: bool ) -> Self
	{
		let (sender, receiver) = unbounded();

		let options = match prevent_default
		{
			true  => EventListenerOptions::enable_prevent_default(),
			false => EventListenerOptions::default(),
		};

		// Attach an event listener
		//
		let _listener = EventListener::new_with_options( &target, event, options, move |event|
		{
			sender.unbounded_send(event.clone()).unwrap_throw();
		});

		Self
		{
			receiver,
			_listener,
		}
	}
}



impl Stream for EHandler
{
	type Item = Event;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>>
	{
		Pin::new( &mut self.receiver ).poll_next(cx)
	}
}

