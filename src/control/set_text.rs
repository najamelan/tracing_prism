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
		let timestamp = get_id( "timestamp" );


		self.lines = match Self::try_json( &msg.text )
		{
			Ok(lines) =>
			{
				self.format = Some( TextFormat::Json );
				timestamp.class_list().remove_1( "display_none" ).expect_throw( "add class to timestamp" );

				Some(lines)
			}


			Err(_) =>
			{
				self.format = Some( TextFormat::Plain );
				timestamp.class_list().add_1( "display_none" ).expect_throw( "add class to timestamp" );

				Some( msg.text.lines().map( |txt|
				{
					Entry::Plain( PlainEntry::new( txt.to_string() ) )

				}).collect() )
			}
		};


		let block: HtmlElement = document().create_element( "div" ).expect_throw( "create div tag" ).unchecked_into();
		block.set_class_name( "logview" );

		for (i, line) in self.lines.as_ref().unwrap().iter().enumerate()
		{
			let div = line.html();
			div.set_id( &format!("e{}", i) );
			block.append_child( &div ).expect_throw( "append p" );
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


impl Control
{
	fn try_json( text: &str ) -> Result< Vec<Entry>, () >
	{
		let mut out = Vec::new();

		for line in text.lines()
		{
			let entry = match JsonEntry::new( line.to_string() )
			{
				Ok( entry ) => Entry::Json( entry ) ,
				_           => return Err(())       ,
			};

			out.push( entry );
		}

		Ok( out )
	}
}
