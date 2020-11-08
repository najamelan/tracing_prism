use crate::{ *, import::* };



#[ derive( Debug, Clone ) ]
//
pub struct ToggleEntry
{
	pub id: String
}


impl Message for ToggleEntry { type Return = (); }


impl Handler<ToggleEntry> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: ToggleEntry )
	{
		let table = self.logview()

			.expect_throw( "logview to exist" )
			.query_selector( &format!( "#{} > table", msg.id ) )
			.expect_throw( "entry to have table" )
			.expect_throw( "entry to have table" )
		;

		table.class_list().toggle( "display_none" ).expect_throw( "to be able to toggle a class" );
	}

	#[async_fn] fn handle( &mut self, _msg: ToggleEntry )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
