use crate::{ *, import::*, column::Column };



#[ derive( Debug, Copy, Clone, PartialEq, Eq ) ]
//
pub enum Show
{
	/// display: none
	//
	None,

	/// visibility: hidden
	//
	Hidden,

	/// visibility: visible, display: block
	//
	Visible,
}




#[ derive( Actor ) ]
//
pub struct Control
{
	logview: Option < HtmlElement         > ,
	lines  : Option < Vec<Entry>          > ,
	show   : HashMap< usize, Vec<Show>    > ,
	columns: HashMap< usize, Addr<Column> > ,
	filters: HashMap< usize, Filter       > ,
}


impl Control
{
	pub fn new() -> Self
	{
		Self
		{
			logview : None           ,
			lines   : None           ,
			show    : HashMap::new() ,
			columns : HashMap::new() ,
			filters : HashMap::new() ,
		}
	}


	#[ must_use = "update the other columns" ]
	//
	pub fn filter
	(
		lines : &Option< Vec<Entry> >            ,
		show  : &mut HashMap< usize, Vec<Show> > ,
		filter: &mut Filter                      ,
		mut all_have_filters: bool               ,
	)

		-> HashMap< usize, bool >
	{
		let mut columns_to_update = HashMap::new();


		// Nothing to filter if no text has yet been set.
		//
		let text = match &lines
		{
			None    => return columns_to_update ,
			Some(v) => v                        ,
		};


		let mut vis = Vec::with_capacity( text.len() );

		// The other columns visibility settings.
		//
		let mut others: Vec<(usize, &mut Vec<Show>)> = Vec::with_capacity( show.len() );

		for (k, v) in show.iter_mut()
		{
			if *k != filter.id
			{
				// When setting a new text, not all columns might have been processed yet.
				// In any case, it only makes sense to do the cross column checks on the last one.
				//
				if v.is_empty()
				{
					all_have_filters = false;
					break;
				}

				others.push( (*k,v) );
			}

		}



		// for each line
		//
		for (i, e) in text.iter().enumerate()
		{
			// check whether we show or not according to filter
			//
			match e.should_show( filter )
			{
				true =>
				{
					vis.push( Show::Visible );

					// We will be visible. Make sure no other column has this set to display:none.
					// We only do cross column logic if every column has a filter set, since otherwise
					// there is at least one column wich will show all lines.
					//
					if all_have_filters
					{
						for other in &mut others
						{
							if other.1[i] == Show::None
							{
								other.1[i] = Show::Hidden;
								columns_to_update.insert( other.0, true );
							}
						}
					}
				}


				false =>
				{
					// if not shown, check whether any other column shows it
					//
					if all_have_filters
					{
						let mut display_none = true;

						for other in &others
						{
							if other.1[i] == Show::Visible
							{
								display_none = false;
							}
						}

						if display_none
						{
							// nobody actually shows the line, display: none.
							//
							vis.push( Show::None );

							for other in &mut others
							{
								if other.1[i] == Show::Hidden
								{
									other.1[i] = Show::None;
									columns_to_update.insert( other.0, true );
								}
							}
						}

						// some columns show the line.
						//
						else
						{
							vis.push( Show::Hidden );
						}
					}


					// not all columns have filters, just hide
					//
					else
					{
						vis.push( Show::Hidden );
					}
				},
			}
		}

		// drop( others );

		show.insert( filter.id, vis );

		columns_to_update
	}
}


// A column is always created with an empty filter. So we send back unfiltered text.
//
pub struct InitColumn( pub Addr<Column> );

impl Message for InitColumn { type Return = (); }


impl Handler<InitColumn> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: InitColumn )
	{
		let mut addr = msg.0;

		if let Some( block ) = &self.logview
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
			Entry::new( txt.to_string() )

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
		for (_, show) in &mut self.show
		{
			show.clear();
		}



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


#[ derive( Debug, Clone, PartialEq, Eq ) ]
//
pub struct Filter
{
	pub id   : usize  ,
	pub txt  : String ,
	pub trace: bool   ,
	pub debug: bool   ,
	pub info : bool   ,
	pub warn : bool   ,
	pub error: bool   ,
}

impl Message for Filter { type Return = (); }


impl Handler<Filter> for Control
{
	#[async_fn_local] fn handle_local( &mut self, mut msg: Filter )
	{
		self.filters.insert( msg.id, msg.clone() );

		let all_have_filters = self.columns.len() == self.filters.len();
		let update = Self::filter( &self.lines, &mut self.show, &mut msg, all_have_filters );

		let col = self.columns.get_mut( &msg.id ).expect_throw( "Handler<Filter>: column to exist" );


		// only tell columns to filter if there is text.
		//
		if self.logview.is_some()
		{
			col.send( Update
			{
				block : None,
				filter: self.show.get( &msg.id ).map( Clone::clone )

			}).await.expect_throw( "send" );
		}

		// Update the other columns to hide lines nobody shows.
		//
		for id in update.keys()
		{
			let col = self.columns.get_mut( &id ).expect_throw( "Handler<SetText> for Control - column to exist" );

			col.send( Update
			{
				block: None,
				filter: self.show.get( &col.id() ).map( Clone::clone ),

			}).await.expect_throw( "send textblock to column" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: Filter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}




impl Handler<DelColumn> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		self.columns.remove( &msg.id );
		self.show   .remove( &msg.id );
		self.filters.remove( &msg.id );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}

