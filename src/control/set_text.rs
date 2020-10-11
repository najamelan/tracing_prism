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
		let entries: Vec<Entry> = msg.text.lines().map( |txt|
		{
			Entry::new( txt.to_string(), self.columns.len() )

		}).collect();


		let block: HtmlElement = document().create_element( "div" ).expect_throw( "create div tag" ).unchecked_into();
		block.set_class_name( "logview" );

		for line in &entries
		{
			let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();

			let class = match line.lvl
			{
				LogLevel::Trace   => "trace"          ,
				LogLevel::Debug   => "debug"          ,
				LogLevel::Info    => "info"           ,
				LogLevel::Warn    => "warn"           ,
				LogLevel::Error   => "error"          ,
				LogLevel::Unknown => "unknown_loglvl" ,
			};

			p.class_list().add_1( class ).expect_throw( "add class to p" );
			p.set_inner_text( &line.txt );
			block.append_child( &p ).expect_throw( "append p" );
		}


		// As we are setting a new text, make sure there are no more show filters around.
		//
		self.show.clear();


		let mut update = HashMap::new();


		// must happen before the loop as we call filter
		//
		self.lines = Some( entries );
		let all_have_filters = self.columns.len() == self.filters.len();

		for col in self.columns.values_mut()
		{
			// If there are pre-existing filters, process the text.
			//
			if let Some(mut filter) = self.filters.get_mut( &col.id() )
			{
				update = Self::filter( &self.lines, &mut self.show, &mut filter, all_have_filters );
			}

			// Send the new text to each column with the last filter we have.
			//
			col.send( Update
			{
				block: Some( block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into() ),
				filter: self.show.get( &col.id() ).map( Clone::clone ),

			}).await.expect_throw( "send textblock to column" );
		}

		for id in update.keys()
		{
			let col = self.columns.get_mut( &id ).expect_throw( "Handler<SetText> for Control - column to exist" );

			col.send( Update
			{
				block: None,
				filter: self.show.get( &col.id() ).map( Clone::clone ),

			}).await.expect_throw( "send textblock to column" );
		}


		self.logview = Some( block );
	}

	#[async_fn] fn handle( &mut self, _msg: SetText )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
