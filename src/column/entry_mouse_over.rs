use crate::{ *, import::* };



#[ derive( Debug, Clone ) ]
//
pub struct EntryMouseOver
{
	pub evt: SendWrapper<Event>
}


impl Message for EntryMouseOver { type Return = (); }


impl Handler<EntryMouseOver> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: EntryMouseOver )
	{
		if let Some(entry) = self.find_entry( msg.evt.target().expect_throw( "event has target" ) )
		{
			if let Some(time) = entry.get_attribute( "data-time" )
			{
				self.columns.send( CurrentTime{ time } ).await.expect_throw( "send" );
			}
		}
	}

	#[async_fn] fn handle( &mut self, _msg: EntryMouseOver )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
