use crate::{ *, import::*, column::Column };

mod filter      ;
mod init_column ;
mod set_text    ;

pub use
{
	filter      :: *,
	init_column :: *,
	set_text    :: *,
};


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


	/// Process a filter change. Returns a hashmap with other columns that need to
	/// recalculate lines that where display: none vs visibility: hidden.
	///
	#[ must_use = "update the other columns" ]
	//
	pub fn filter
	(
		lines : &Option< Vec<Entry> >            ,
		show  : &mut HashMap< usize, Vec<Show> > ,
		filter: &mut Filter                      ,
		all_have_filters: bool                   ,
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
				others.push( (*k,v) );
			}

		}



		// for each line
		//
		for (i, e) in text.iter().enumerate()
		{
			// check whether we show or not according to filter
			//
			match e.matches( filter )
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

