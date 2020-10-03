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


	pub fn filter( lines: &Option< Vec<Entry> >, show: &mut HashMap< usize, Vec<Show> >, filter: &mut Filter )
	{
		match &lines
		{
			None => return,

			Some(vec) =>
			{
				let vis = match show.get_mut( &filter.id )
				{
					None =>
					{
						show.insert( filter.id, Vec::new() );
						show.get_mut( &filter.id ).expect_throw( "Control::filter - column exists" )
					}

					Some(v) =>
					{
						v.clear();
						v
					}
				};


				for e in vec
				{
					// check whether we show or not according to filter
					//
					match e.show( filter )
					{
						true => vis.push( Show::Visible ) ,

						// if not shown, check whether any other column shows it
						//
						false =>
						{
							vis.push( Show::Hidden );
						},
					}
				}
			}
		}
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


		// must happen before the loop as we call filter
		//
		self.lines = Some( entries );


		for col in self.columns.values_mut()
		{
			// If there are pre-existing filters, process the text.
			//
			if let Some(mut filter) = self.filters.get_mut( &col.id() )
			{
				Self::filter( &self.lines, &mut self.show, &mut filter );
			}

			// Send the new text to each column with the last filter we have.
			//
			col.send( Update
			{
				block: Some( block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into() ),
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
		Self::filter( &self.lines, &mut self.show, &mut msg );

		let col = self.columns.get_mut( &msg.id ).expect_throw( "Handler<Filter>: column to exist" );


		// only tell columns to filter if there is text.
		//
		if self.logview.is_some()
		{
			col.send( Update
			{
				block : None,
				filter: self.show.get( &msg.id ).map( |f| f.clone() )

			}).await.expect_throw( "send" );
		}


		self.filters.insert( msg.id, msg );
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

