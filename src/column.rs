use crate::{ import::*, * };


#[ derive( Actor, Clone ) ]
//
pub struct Column
{
	parent   : HtmlElement   , // #columns
	container: HtmlElement   , // .column
	columns  : Addr<Columns> ,
	addr     : Addr<Self>    ,
	filter   : String        ,
	trace    : bool          ,
	debug    : bool          ,
	info     : bool          ,
	warn     : bool          ,
	error    : bool          ,
}


impl Column
{
	pub fn new( parent: HtmlElement, addr: Addr<Self>, columns: Addr<Columns> ) -> Self
	{
		let container: HtmlElement = document().create_element( "div" ).expect_throw( "create div" ).unchecked_into();
		container.set_class_name( "column" );

		Self
		{
			parent                ,
			container             ,
			addr                  ,
			columns               ,
			filter: String::new() ,
			trace : true          ,
			debug : true          ,
			info  : true          ,
			warn  : true          ,
			error : true          ,
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


	pub fn logview( &self ) -> Option<HtmlElement>
	{
		self.container.query_selector( ".logview" ).expect_throw( "query_selector" ).map( |e| e.unchecked_into() )
	}


	async fn on_filter( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ChangeFilter ) ).forward( column ).await.expect_throw( "send ChangeFilter" );
	}


	async fn on_trace( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ToggleTrace ) ).forward( column ).await.expect_throw( "send ToggleTrace" );
	}


	async fn on_debug( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ToggleDebug ) ).forward( column ).await.expect_throw( "send ToggleDebug" );
	}


	async fn on_info( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ToggleInfo ) ).forward( column ).await.expect_throw( "send ToggleInfo" );
	}


	async fn on_warn( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ToggleWarn ) ).forward( column ).await.expect_throw( "send ToggleWarn" );
	}


	async fn on_error( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( ToggleError ) ).forward( column ).await.expect_throw( "send ToggleError" );
	}


	async fn on_collapse( evts: impl Stream< Item=Event > + Unpin, column: Addr<Column> )
	{
		evts.map( |_| Ok( Collapse ) ).forward( column ).await.expect_throw( "send Collapse" );
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


	fn filter_text( &self )
	{
		let children = self.find( ".logview" ).children();

		let mut i = 0;


		while i < children.length()
		{
			let p: HtmlElement = children.item( i ).expect_throw( "get p" ).unchecked_into();
			let text = p.text_content().expect_throw( "text_content" ).to_lowercase();
			let mut hide = false;


			if !self.filter.is_empty() && !text.contains( &self.filter )
			{
				hide = true;
			}


			// TODO: verify rust does lazy evaluation of conditions.
			//
			if !self.trace && text.contains( "trace" ) { hide = true; }
			if !self.debug && text.contains( "debug" ) { hide = true; }
			if !self.info  && text.contains( "info"  ) { hide = true; }
			if !self.warn  && text.contains( "warn"  ) { hide = true; }
			if !self.error && text.contains( "error" ) { hide = true; }


			if hide
			{
				p.style().set_property( "visibility", "hidden" ).expect_throw( "hide line" );
			}

			else
			{
				p.style().set_property( "visibility", "visible" ).expect_throw( "show line" );
			}

			i += 1;
		}
	}
}




pub struct TextBlock
{
	pub block: HtmlElement,
}

unsafe impl Send for TextBlock {}

impl Message for TextBlock { type Return = (); }



impl Handler<TextBlock> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: TextBlock )
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

		self.container.append_child( &msg.block ).expect_throw( "append div" );
		self.filter_text();

		// set display none if column is collapsed.
		//
		let controls = self.find( ".col-controls" );


		if controls.class_list().contains( "collapsed" )
		{
			msg.block.style().set_property( "display", "none" ).expect_throw( "set display none" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: TextBlock )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
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


		// Set event listeners on buttons
		//
		let filter      = self.find( ".filter-input" );
		let filter_evts = EHandler::new( &filter, "input", false );

		spawn_local( Column::on_filter( filter_evts, self.addr.clone() ) );


		let trace_but  = self.find( ".button-trace" );
		let trace_evts = EHandler::new( &trace_but, "click", false );

		spawn_local( Column::on_trace( trace_evts, self.addr.clone() ) );


		let debug_but  = self.find( ".button-debug" );
		let debug_evts = EHandler::new( &debug_but, "click", false );

		spawn_local( Column::on_debug( debug_evts, self.addr.clone() ) );


		let info_but  = self.find( ".button-info" );
		let info_evts = EHandler::new( &info_but, "click", false );

		spawn_local( Column::on_info( info_evts, self.addr.clone() ) );


		let warn_but  = self.find( ".button-warn" );
		let warn_evts = EHandler::new( &warn_but, "click", false );

		spawn_local( Column::on_warn( warn_evts, self.addr.clone() ) );


		let error_but  = self.find( ".button-error" );
		let error_evts = EHandler::new( &error_but, "click", false );

		spawn_local( Column::on_error( error_evts, self.addr.clone() ) );


		let del_col  = self.find( ".button-close" );
		let del_evts = EHandler::new( &del_col, "click", false );

		spawn_local( Column::on_delcol( del_evts, self.addr.clone() ) );


		let collapse      = self.find( ".button-collapse" );
		let collapse_evts = EHandler::new( &collapse, "click", false );

		spawn_local( Column::on_collapse( collapse_evts, self.addr.clone() ) );
	}


	#[async_fn] fn handle( &mut self, _msg: Render )
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

		self.filter = filter.value().to_lowercase();


		if self.logview().is_some()
		{
			self.filter_text();
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ChangeFilter )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



pub struct ToggleTrace;

impl Message for ToggleTrace { type Return = (); }


impl Handler<ToggleTrace> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleTrace )
	{
		self.trace = !self.trace;

		if self.logview().is_some()
		{
			self.filter_text();
		}

		let button = self.find( ".button-trace" );

		if self.trace
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



pub struct ToggleDebug;

impl Message for ToggleDebug { type Return = (); }


impl Handler<ToggleDebug> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleDebug )
	{
		self.debug = !self.debug;

		if self.logview().is_some()
		{
			self.filter_text();
		}

		let button = self.find( ".button-debug" );

		if self.debug
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



pub struct ToggleInfo;

impl Message for ToggleInfo { type Return = (); }


impl Handler<ToggleInfo> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleInfo )
	{
		self.info = !self.info;

		if self.logview().is_some()
		{
			self.filter_text();
		}

		let button = self.find( ".button-info" );

		if self.info
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



pub struct ToggleWarn;

impl Message for ToggleWarn { type Return = (); }


impl Handler<ToggleWarn> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleWarn )
	{
		self.warn = !self.warn;

		if self.logview().is_some()
		{
			self.filter_text();
		}

		let button = self.find( ".button-warn" );

		if self.warn
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



pub struct ToggleError;

impl Message for ToggleError { type Return = (); }


impl Handler<ToggleError> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleError )
	{
		self.error = !self.error;

		if self.logview().is_some()
		{
			self.filter_text();
		}

		let button = self.find( ".button-error" );

		if self.error
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
