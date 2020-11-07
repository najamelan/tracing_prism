use crate::{ *, import::* };


pub struct ChangeFilter;

impl Message for ChangeFilter { type Return = (); }


impl Handler<ChangeFilter> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ChangeFilter )
	{

		let filter: HtmlInputElement = self.find( ".filter-input" ).unchecked_into();

		let new = filter.value();

		// Don't do anything if the text hasn't changed. This can happen when the user
		// types faster than we can process. Then several events will be fired before we
		// read the input value. However, we will still get to process the other events.
		//
		if &self.filter_txt != &new
		{
			self.filter.regex = Some( Regex::new( &format!( "(?i){}", &new ) ).expect_throw( "valid regex" ) );
			self.filter_txt   = new;

			// This provides the back pressure.
			//
			self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ChangeFilter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
