use crate::{ *, import::*, ToggleEntry };



#[ derive( Debug, Clone ) ]
//
pub struct EntryClick
{
	pub evt: SendWrapper<Event>
}


impl Message for EntryClick { type Return = (); }


impl Handler<EntryClick> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: EntryClick )
	{
		if is_text_selected() { return; }


		if let Some(entry) = self.find_entry( msg.evt.target().expect_throw( "event has target" ) )
		{
			let id = entry.id();

			self.control.send( ToggleEntry{ id } ).await.expect_throw( "send" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: EntryClick )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
