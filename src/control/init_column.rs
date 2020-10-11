use crate::{ *, import::* };

// A column is always created with an empty filter. So we send back unfiltered text.
//
pub struct InitColumn( pub Addr<Column> );

impl Message for InitColumn { type Return = (); }


impl Handler<InitColumn> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: InitColumn )
	{
		let mut addr = msg.0;

		if let Some(text) = &mut self.lines
		{
			for mut entry in text
			{
				entry.shown += 1;
			}

			// Since we are adding a new column which will be showing all text,
			// we need to remove all display: none.
			//
			for (id, col_addr) in &mut self.columns
			{
				if let Some(show) = self.show.get_mut( &id )
				{
					for vis in show.iter_mut()
					{
						if vis == &Show::None
						{
							*vis = Show::Hidden;
						}
					}


					col_addr.send( Update
					{
						block : None,
						filter: Some( show.clone() ),

					}).await.expect_throw( "send textblock to column" );
				}
			}
		};


		if let Some(block) = &self.logview
		{
			addr.send( Update
			{
				block : Some( block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into() ),
				filter: None, // as the column is brand new, no filters yet.

			}).await.expect_throw( "send textblock to column" );
		}

		self.columns.insert( addr.id(), addr );
	}

	#[async_fn] fn handle( &mut self, _msg: InitColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}

