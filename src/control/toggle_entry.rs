use crate::{ *, import::*, ToggleEntry };

impl Handler<ToggleEntry> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: ToggleEntry )
	{
		for column in self.columns.values_mut()
		{
			column.send( msg.clone() ).await.expect_throw( "send" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: ToggleEntry )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
