use crate::{ *, import::* };


pub struct SetText
{
	pub text: String,
}

impl Message for SetText { type Return = (); }



impl Handler<SetText> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: SetText )
	{
		self.lines = Some( msg.text.lines().map( |txt|
		{
			match JsonEntry::new( txt.to_string() )
			{
				Ok ( entry ) => entry,
				err          =>
				{
					error!( "{}", txt );
					err.expect_throw( "create json entry" );
					panic!();
				},
			}

		}).collect() );

		let block: HtmlElement = document().create_element( "div" ).expect_throw( "create div tag" ).unchecked_into();
		block.set_class_name( "logview" );

		for line in self.lines.as_ref().unwrap()
		{
			block.append_child( &line.html() ).expect_throw( "append p" );
		}

		// As we are setting a new text, make sure there are no more show filters around.
		//
		self.show.clear();

		self.logview = Some( block );

		self.update_all().await;
	}


	#[async_fn] fn handle( &mut self, _msg: SetText )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
