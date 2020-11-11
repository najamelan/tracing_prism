use crate::{ *, import::* };


pub struct AddColumn;

impl Message for AddColumn { type Return = (); }


impl Handler<AddColumn> for Columns
{
	#[async_fn_local] fn handle_local( &mut self, _msg: AddColumn )
	{
		let (mut addr, mb) = Addr::builder().build();
		let col            = Column::new( self.container.clone(), addr.clone(), self.addr_columns.clone(), self.addr_control.clone() );

		spawn_local( async{ mb.start_local( col ).await; } );

		addr.send( Render ).await.expect_throw( "send render to column" );

		self.children.insert( addr.id(), addr );
	}

	#[async_fn] fn handle( &mut self, _msg: AddColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
