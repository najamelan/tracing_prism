use crate::{ *, import::* };



#[ derive( Debug, Clone ) ]
//
pub struct CurrentTime
{
	pub time: String
}

impl Message for CurrentTime { type Return = (); }


impl Handler<CurrentTime> for Columns
{
	#[async_fn_local] fn handle_local( &mut self, msg: CurrentTime )
	{
		let span = get_id( "timestamp" );

		span.set_inner_text( &msg.time );
	}

	#[async_fn] fn handle( &mut self, _msg: CurrentTime )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
