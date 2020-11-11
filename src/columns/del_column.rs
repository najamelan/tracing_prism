use crate::{ *, import::* };



impl Handler<DelColumn> for Columns
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		self.children.remove( &msg.id );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
