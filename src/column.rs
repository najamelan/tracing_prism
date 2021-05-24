use crate::{ import::*, * };

mod change_filter;
mod collapse;
mod del_column;
mod entry_click;
mod entry_mouse_over;
mod render;
mod toggle;
mod toggle_entry;
mod update;


pub use change_filter    ::*;
pub use collapse         ::*;
pub use del_column       ::*;
pub use entry_click      ::*;
pub use entry_mouse_over ::*;
pub use render           ::*;
pub use toggle           ::*;
pub use toggle_entry     ::*;
pub use update           ::*;


#[ derive( Actor ) ]
//
pub struct Column
{
	parent         : HtmlElement              , // #columns
	container      : HtmlElement              , // .column
	columns        : Addr<Columns>            ,
	addr           : Option< Addr<Self> >     ,
	control        : Addr<Control>            ,
	filter         : Filter                   ,
	nursery        : Nursery< Bindgen, () >   ,
	nursery_output : Option< JoinHandle<()> > ,
}


impl Column
{
	pub fn new( parent: HtmlElement, addr: Addr<Self>, columns: Addr<Columns>, control: Addr<Control> ) -> Self
	{
		let container: HtmlElement = document().create_element( "div" ).expect_throw( "create div" ).unchecked_into();
		container.set_class_name( "column" );

		let (nursery, nursery_output) = Nursery::new( Bindgen );

		let nursery_output = Some( Bindgen.spawn_handle( nursery_output ).expect_throw( "spawn nursery_output" ) );
		let filter         = Filter::new( addr.id() );

		Self
		{
			addr: Some(addr) ,
			parent           ,
			container        ,
			columns          ,
			control          ,
			filter           ,
			nursery          ,
			nursery_output   ,
		}
	}


	pub fn find( &self, selector: &str ) -> HtmlElement
	{
		let expect = format!( "find {}", &selector );

		self.container

			.query_selector( selector )
			.expect_throw  ( &expect  )
			.expect_throw  ( &expect  )
			.unchecked_into()
	}


	/// Set's the classes to hide and display none on lines.
	//
	fn filter( &self, logview: &HtmlElement, filter: &[Show] )
	{
		let children = logview.children();
		let mut odd  = true;

		let mut i = 0;
		let length = children.length();

		while i < length
		{
			let p: HtmlElement = children.item( i ).expect_throw( "get p" ).unchecked_into();
			let class = p.class_list();

			match filter[i as usize]
			{
				Show::None =>
				{
					class.add_1( "hidden"       ).expect_throw( "add hidden class"       );
					class.add_1( "display_none" ).expect_throw( "add display_none class" );
				}

				Show::Visible =>
				{
					class.remove_1( "hidden"       ).expect_throw( "remove hidden class"       );
					class.remove_1( "display_none" ).expect_throw( "remove display_none class" );

					match odd
					{
						true  => class.add_1   ( "odd" ).expect_throw( "add odd class"    ) ,
						false => class.remove_1( "odd" ).expect_throw( "remove odd class" ) ,
					}

					odd ^= true; // .toggle()
				}

				Show::Hidden =>
				{
					class.add_1   ( "hidden"       ).expect_throw( "add hidden class"          );
					class.remove_1( "display_none" ).expect_throw( "remove display_none class" );

					match odd
					{
						true  => class.add_1   ( "odd" ).expect_throw( "add odd class"    ) ,
						false => class.remove_1( "odd" ).expect_throw( "remove odd class" ) ,
					}

					odd ^= true; // .toggle()
				}
			}

			i += 1;
		}
	}



	pub fn logview( &self ) -> Option<HtmlElement>
	{
		self.container.query_selector( ".logview" ).expect_throw( "query_selector" ).map( |e| e.unchecked_into() )
	}


	async fn on_filter( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ChangeFilter ) ).forward( column ).await.expect_throw( "send ChangeFilter" );
	}


	async fn on_delcol
	(
		mut evts: impl Stream< Item=Event > + Unpin ,
		mut column: Addr<Column>,
	)
	{
		evts.next().await;
		column.send( DelColumn{ id: column.id() } ).await.expect_throw( "send DelColumn" );
	}


	async fn on_use_regex
	(
		evts  : impl Stream< Item=Event > + Unpin ,
		column: Addr<Column>,
	)
	{
		evts.map( |_| Ok( ChangeFilter ) ).forward( column ).await.expect_throw( "send ChangeFilter" );
	}


	async fn on_case_sensitive
	(
		evts  : impl Stream< Item=Event > + Unpin ,
		column: Addr<Column>,
	)
	{
		evts.map( |_| Ok( ChangeFilter ) ).forward( column ).await.expect_throw( "send ChangeFilter" );
	}


	// TODO: can we leak memory? Eg. does the event listener get dropped when the element gets
	// removed from the dom?
	//
	async fn on_click_entry
	(
		mut evts: impl Stream< Item=Event > + Unpin ,
		mut column: Addr<Column>,
	)
	{
		while let Some(evt) = evts.next().await
		{
			let evt = SendWrapper::new( evt );
			column.call( EntryClick{ evt } ).await.expect_throw( "send EntryClick" );
		}
	}


	// TODO: can we leak memory? Eg. does the event listener get dropped when the element gets
	// removed from the dom?
	//
	async fn on_mouse_over_entry
	(
		mut evts: impl Stream< Item=Event > + Unpin ,
		mut column: Addr<Column>,
	)
	{
		while let Some(evt) = evts.next().await
		{
			let evt = SendWrapper::new( evt );
			column.call( EntryMouseOver{ evt } ).await.expect_throw( "send EntryClick" );
		}
	}


	/// Set's up the event handlers for the loglevel filter buttons.
	//
	fn toggle_button<M>( &mut self, elem: &str )

		where Self: Handler<M>,
		      M   : Message + Default + std::fmt::Debug
	{
		let button = self.find( elem );
		let evts   = EHandler::new( &button, "click", true ).map( |_| Ok( M::default() ) );
		let addr   = self.addr.as_ref().expect_throw( "Column unwrap addr" ).clone();

		let task = async move
		{
			evts.forward( addr ).await.expect_throw( &format!( "send {:?}", M::default() ) );
		};

		self.nursery.nurse_local( task ).expect_throw( "Column::toggle_button - spawn" );
	}


	fn find_entry( &self, target: EventTarget ) -> Option<HtmlElement>
	{
		let mut target: HtmlElement = target.dyn_into().expect( "HtmlElement" );

		// We can click between entries and thus end up on the logview. In that case disregard the click.
		//
		if target.class_list().contains( "logview" ) { return None; }


		// target could be a descendant of entry, walk the tree.
		//
		if !target.class_list().contains( "entry" )
		{
			while let Some( element ) = target.parent_node()
			{
				let element: HtmlElement = element.dyn_into().expect( "HtmlElement" );

				if target.class_list().contains( "logview" ) { return None; }

				if element.class_list().contains( "entry" )
				{
					target = element;

					break;
				}

				target = element
			}
		}

		Some(target)
	}
}


impl Drop for Column
{
	fn drop( &mut self )
	{
		debug!( "drop Column" );
	}
}
