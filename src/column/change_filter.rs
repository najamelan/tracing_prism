use crate::{ *, import::* };


pub struct ChangeFilter;

impl Message for ChangeFilter { type Return = (); }


impl Handler<ChangeFilter> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ChangeFilter )
	{

		let filter     : HtmlInputElement = self.find( ".filter-input" ).unchecked_into();
		let use_regex  : HtmlInputElement = get_id( "use-regex"        ).unchecked_into();
		let case       : HtmlInputElement = get_id( "case"             ).unchecked_into();


		let new = match use_regex.checked()
		{
			true  => filter.value()                 ,
			false => regex::escape(&filter.value()) ,
		};


		let regex = match case.checked()
		{
			true  => Regex::new( &new                     ) ,
			false => Regex::new( &format!("(?i){}", &new) ) ,
		};


		let regex = match regex
		{
			Ok(re) =>
			{
				filter.class_list().remove_1( "wrong-regex" ).expect_throw( "remove wrong-regex class" );

				Some(re)
			}

			Err(_) =>
			{
				filter.class_list().add_1( "wrong-regex" ).expect_throw( "add wrong-regex class" );

				return;
			}
		};


		// Don't do anything if the regex hasn't changed. This can happen when the user
		// types faster than we can process. Then several events will be fired before we
		// read the input value. However, we will still get to process the other events.
		//
		if self.filter.regex.as_ref().map(Regex::as_str) != regex.as_ref().map(Regex::as_str)
		{
			self.filter.regex = regex;

			// This provides the back pressure.
			//
			self.control.call( self.filter.clone() ).await.expect_throw( "update filter" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ChangeFilter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
