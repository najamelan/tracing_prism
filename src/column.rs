use crate::{ import::*, * };


#[ derive( Actor, Clone ) ]
//
pub struct Column
{
	parent    : HtmlElement   , // #columns
	container : HtmlElement   , // .column
	columns   : Addr<Columns> ,
	addr      : Addr<Self>    ,
	control   : Addr<Control> ,
	filter    : Filter        ,
}


impl Column
{
	pub fn new( parent: HtmlElement, addr: Addr<Self>, columns: Addr<Columns>, control: Addr<Control> ) -> Self
	{
		let container: HtmlElement = document().create_element( "div" ).expect_throw( "create div" ).unchecked_into();
		container.set_class_name( "column" );

		let filter = Filter
		{
			id: addr.id(),
			trace: true,
			debug: true,
			info : true,
			warn : true,
			error: true,
			txt  : String::new(),
		};

		Self
		{
			parent    ,
			container ,
			addr      ,
			columns   ,
			control   ,
			filter    ,
		}
	}


	pub fn find( &self, selector: &str ) -> HtmlElement
	{
		let expect = format!( "find {}", &selector );

		self.container

			.query_selector( selector )
			.expect_throw( &expect )
			.expect_throw( &expect )
			.unchecked_into()
	}


	/// Set's the classes to hide and display none on lines.
	//
	fn filter( &self, filter: &Vec<Show> )
	{
		let children = self.find( ".logview" ).children();

		let mut i = 0;
		let length = children.length();

		while i < length
		{
			let p: HtmlElement = children.item( i ).expect_throw( "get p" ).unchecked_into();

			match filter[i as usize]
			{
				Show::None =>
				{
					p.style().set_property( "visibility", "hidden" ).expect_throw( "hide line" );
				}

				Show::Visible =>
				{
					p.style().set_property( "visibility", "visible" ).expect_throw( "hide line" );
				}

				Show::Hidden =>
				{
					p.style().set_property( "visibility", "hidden" ).expect_throw( "hide line" );
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


	/// Set's up the event handlers for the loglevel filter buttons.
	//
	fn toggle_button<M>( &mut self, elem: &str )

		where Self: Handler<M>,
		      M   : Message + Default + std::fmt::Debug
	{
		let button = self.find( elem );
		let evts   = EHandler::new( &button, "click", true ).map( |_| Ok( M::default() ) );
		let addr   = self.addr.clone();

		let task = async move
		{
			evts.forward( addr ).await.expect_throw( &format!( "send {:?}", M::default() ) );
		};

		// TODO: use nursery and make sure we don't leak when the column is removed.
		//
		spawn_local( task );
	}
}




pub struct Render;

impl Message for Render { type Return = (); }


impl Handler<Render> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: Render )
	{
		let controls: HtmlElement = document().query_selector( ".col-controls.template" )

			.expect_throw( "find col-controls" )
			.expect_throw( "find col-controls" )
			.clone_node_with_deep( true )
			.expect_throw( "clone filter" )
			.unchecked_into()
		;

		controls.set_class_name( "col-controls" );


		self.container.append_child( &controls       ).expect_throw( "append filter" );
		self.parent   .append_child( &self.container ).expect_throw( "append column" );

		self.control.send( InitColumn( self.addr.clone() ) ).await.expect_throw( "send init column" );

		// Set event listeners on buttons
		// TODO: use drop channel
		//
		let filter      = self.find( ".filter-input" );
		let filter_evts = EHandler::new( &filter, "input", true );

		spawn_local( Column::on_filter( filter_evts, self.addr.clone() ) );


		let del_col  = self.find( ".button-close" );
		let del_evts = EHandler::new( &del_col, "click", true );

		spawn_local( Column::on_delcol( del_evts, self.addr.clone() ) );


		self.toggle_button::<Collapse>( ".button-collapse" );

		self.toggle_button::<ToggleTrace>( ".button-trace" );
		self.toggle_button::<ToggleDebug>( ".button-debug" );
		self.toggle_button::<ToggleInfo >( ".button-info"  );
		self.toggle_button::<ToggleWarn >( ".button-warn"  );
		self.toggle_button::<ToggleError>( ".button-error" );
	}


	#[async_fn] fn handle( &mut self, _msg: Render )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}


pub struct Update
{
	pub block : Option< HtmlElement >,
	pub filter: Option< Vec<Show>   >,
}

// TODO: use SendWrapper?
//
unsafe impl Send for Update {}


impl Message for Update { type Return = (); }

impl Handler<Update> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: Update )
	{
		if let Some(block) = &msg.block
		{
			// if the element exists, remove it
			// add the new one
			// filter the new one.

			if let Some(elem) = self.logview()
			{
				// Hopefully leak a bit less memory:
				// https://stackoverflow.com/a/3785323
				// needs some more research.
				//
				elem.set_inner_html( "" );
				elem.remove();
			}

			// set display none if column is collapsed.
			//
			let controls = self.find( ".col-controls" );


			if controls.class_list().contains( "collapsed" )
			{
				block.style().set_property( "display", "none" ).expect_throw( "set display none" );
			}
		}


		if let Some(f) = msg.filter
		{
			self.filter( &f );
		}


		if let Some(block) = msg.block
		{
			self.container.append_child( &block ).expect_throw( "append div" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: Update )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



pub struct DelColumn
{
	pub id: usize
}

impl Message for DelColumn { type Return = (); }


impl Handler<DelColumn> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		self.container.set_inner_html( "" );
		self.container.remove();
		self.columns.send( msg ).await.expect_throw( "send DelColumn to Columns" );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct Collapse;

impl Message for Collapse { type Return = (); }


impl Handler<Collapse> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: Collapse )
	{
		// turn controls sideways
		//
		let controls = self.find( ".col-controls" );


		if self.container.class_list().contains( "collapsed" )
		{
			self.container.class_list().remove_1( "collapsed" ).expect_throw( "remove collapsed class" );

			// set with of column to height of controls
			//
			self.container.style().set_property( "width", "auto" ).expect_throw( "set width" );


			if let Some( logview ) = self.logview()
			{
				logview.style().set_property( "display", "block" ).expect_throw( "set display block" );
			}
		}

		else
		{
			self.container.class_list().add_1( "collapsed" ).expect_throw( "add collapsed class" );

			// set with of column to height of controls
			//
			let width = controls.get_bounding_client_rect().width();
			self.container.style().set_property( "width", &format!( "{}", width ) ).expect_throw( "set width" );


			if let Some( logview ) = self.logview()
			{
				logview.style().set_property( "display", "none" ).expect_throw( "set display none" );
			}
		}
	}


	#[async_fn] fn handle( &mut self, _msg: Collapse )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



pub struct ChangeFilter;

impl Message for ChangeFilter { type Return = (); }


impl Handler<ChangeFilter> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ChangeFilter )
	{
		let filter: HtmlInputElement = self.find( ".filter-input" ).unchecked_into();

		self.filter.txt = filter.value().to_lowercase();

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );
	}


	#[async_fn] fn handle( &mut self, _msg: ChangeFilter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleTrace;

impl Message for ToggleTrace { type Return = (); }


impl Handler<ToggleTrace> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleTrace )
	{
		self.filter.trace = !self.filter.trace;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-trace" );

		if self.filter.trace
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-trace" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-trace" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleTrace )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleDebug;

impl Message for ToggleDebug { type Return = (); }


impl Handler<ToggleDebug> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleDebug )
	{
		self.filter.debug = !self.filter.debug;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-debug" );

		if self.filter.debug
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-debug" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-debug" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleDebug )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleInfo;

impl Message for ToggleInfo { type Return = (); }


impl Handler<ToggleInfo> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleInfo )
	{
		self.filter.info = !self.filter.info;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-info" );

		if self.filter.info
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-info" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-info" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleInfo )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleWarn;

impl Message for ToggleWarn { type Return = (); }


impl Handler<ToggleWarn> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleWarn )
	{
		self.filter.warn = !self.filter.warn;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-warn" );

		if self.filter.warn
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-warn" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-warn" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleWarn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleError;

impl Message for ToggleError { type Return = (); }


impl Handler<ToggleError> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleError )
	{
		self.filter.error = !self.filter.error;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-error" );

		if self.filter.error
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-error" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-error" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleError )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
