use crate::{ *, import::* };



#[ derive( Debug, Default, Copy, Clone, PartialEq, Eq ) ]
//
pub struct DelColumn
{
	pub id: usize
}

impl Message for DelColumn { type Return = (); }


impl Handler<DelColumn> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		// Stop processing input
		//
		drop( self.nursery_output.take() );
		drop( self.addr.take()           );

		self.container.set_inner_html( "" );
		self.container.remove();

		self.columns.send( msg ).await.expect_throw( "send DelColumn to Columns" );
		self.control.send( msg ).await.expect_throw( "send DelColumn to Control" );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
