use crate::{ *, import::* };

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

impl Filter
{
	pub fn new( id: usize ) -> Self
	{
		Self
		{
			id                   ,
			txt  : String::new() ,
			trace: true          ,
			debug: true          ,
			info : true          ,
			warn : true          ,
			error: true          ,
		}
	}
}

impl Message for Filter { type Return = (); }


impl Handler<Filter> for Control
{
	#[async_fn_local] fn handle_local( &mut self, mut msg: Filter )
	{
		self.filters.insert( msg.id, msg.clone() );

		let all    = self.all_have_filters();
		let update = Self::filter( &mut self.lines, &mut self.show, &mut msg, all );

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
		if update
		{
			for (_, col) in self.columns.iter_mut().filter( |(id, _)| **id != msg.id )
			{
				col.send( Update
				{
					block: None,
					filter: self.show.get( &col.id() ).map( Clone::clone ),

				}).await.expect_throw( "send textblock to column" );
			}
		}
	}


	#[async_fn] fn handle( &mut self, _msg: Filter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}


