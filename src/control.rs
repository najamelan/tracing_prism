use crate::{ *, import::*, column::Column };

mod filter      ;
mod init_column ;
mod set_text    ;
mod toggle_entry;

pub use
{
	filter      :: *,
	init_column :: *,
	set_text    :: *,
	toggle_entry:: *,
};



/// Whether to show lines. We distinguish between hiding the text and display: none. When no
/// column shows a certain line, we use display: none to avoid wasting screen space as the user
/// might have to scroll a lot to read the log.
//
#[ derive( Debug, Copy, Clone, PartialEq, Eq ) ]
//
pub enum Show
{
	/// display: none
	//
	None,

	/// visibility: hidden, display: block
	//
	Hidden,

	/// visibility: visible, display: block
	//
	Visible,
}


// The brains of the application.
//
// When the column get filtered, they send their filter settings and this computes
// the display settings for each line.
//
// When a line is hidden in all columns, we set it to display: none so we recover the space.
// This means that we need to re-evaluate if any columns show a line at:
// - set text   -> re-apply existing filters
// - add column -> this one shows all lines by default
// - del column -> any lines that were only shown in this one need to be display: none now in other columns.
// - filter     -> when any filter changes and it makes lines visible, we need to re-evaluate other columns.
//
#[ derive( Actor ) ]
//
pub struct Control
{
	logview: Option < HtmlElement         > ,
	lines  : Option < Vec<Entry>          > ,
	format : Option < TextFormat          > ,
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
			format  : None           ,
			show    : HashMap::new() ,
			columns : HashMap::new() ,
			filters : HashMap::new() ,
		}
	}


	pub fn all_have_filters( &self ) -> bool
	{
		self.columns.len() == self.filters.len()
	}


	/// Process a filter change. Returns a hashmap with other columns that need to
	/// recalculate lines that where display: none vs visibility: hidden.
	///
	#[ must_use = "update the other columns" ]
	//
	pub fn filter
	(
		mut lines       : &mut Option< Vec<Entry> >        ,
		show            : &mut HashMap< usize, Vec<Show> > ,
		filter          : &mut Filter                      ,
		all_have_filters: bool                             ,
	)

		-> bool
	{
		let mut update_others = false;

		// Nothing to filter if no text has yet been set.
		//
		let text = match &mut lines
		{
			None    => return update_others ,
			Some(v) => v                    ,
		};


		// We take the show information for the column who's filter changed out of the
		// hashmap, so we can also operate on the other columns. At the end we will put
		// it back.
		//
		// If it doesnt exist yet, we create a new one.
		//
		let mut vis =

			if   let Some( v ) = show.remove( &filter.id ) { v }


			else
			{
				// this is silly, but we have to initialize to do index access below.
				//
				vec![ Show::Visible; text.len() ]
			}
		;


		// for each line
		//
		for (i, e) in text.iter_mut().enumerate()
		{
			// check whether we show or not according to filter
			//
			match e.matches( filter )
			{
				true =>
				{
					vis[i] = Show::Visible;

					// We will be visible. Make sure no other column has this set to display:none.
					// We only do cross column logic if every column has a filter set, since otherwise
					// there is at least one column wich will show all lines.
					//
					if all_have_filters
					{
						for other in show.values_mut()
						{
							if other[i] == Show::None
							{
								other[i] = Show::Hidden;
								update_others = true;
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

						for other in show.values()
						{
							if other[i] == Show::Visible
							{
								display_none = false;
							}
						}


						// No other column shows it, use display: none.
						//
						if display_none
						{
							vis[i] = Show::None;

							for other in show.values_mut()
							{
								if other[i] == Show::Hidden
								{
									other[i] = Show::None;
									update_others = true;
								}
							}
						}


						// some columns show the line.
						//
						else
						{
							vis[i] = Show::Hidden;
						}
					}


					// not all columns have filters, just hide
					//
					else
					{
						vis[i] = Show::Hidden;
					}
				},
			}
		}


		show.insert( filter.id, vis );
		update_others
	}


	/// Recalculate the show information for all columns.
	//
	async fn update_all( &mut self )
	{
		let all_have_filters = self.all_have_filters();

		// Generate show information for each column.
		//
		for col in self.columns.values_mut()
		{
			// If there are pre-existing filters, process the text.
			//
			if let Some(mut filter) = self.filters.get_mut( &col.id() )
			{
				// we ignore the return as we will update all columns anyway, since there is a new text.
				//
				let _ = Self::filter( &mut self.lines, &mut self.show, &mut filter, all_have_filters );
			}
		}


		// update the text and show information for each column.
		//
		// It is important there are 2 loops, as processing a filter might change other columns.
		//
		for col in self.columns.values_mut()
		{
			let block = self.logview.as_ref().map( |logview|

				logview
					.clone_node_with_deep( true )
					.expect_throw( "clone text" )
					.unchecked_into()
			);

			let filter = self.show.get( &col.id() ).map( Clone::clone );

			// Send the new text to each column with the last filter we have.
			//
			col.send( Update
			{
				block,
				filter,
				format: self.format.expect_throw( "have a text format set" ),

			}).await.expect_throw( "send textblock to column" );
		}
	}
}





impl Handler<DelColumn> for Control
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		self.columns.remove( &msg.id );
		self.show   .remove( &msg.id );
		self.filters.remove( &msg.id );

		self.update_all().await;
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}

