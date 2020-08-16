use crate::{ import::*, * };


#[ derive( Actor, Clone ) ]
//
pub struct Column
{
	parent   : HtmlElement   ,
	container: HtmlElement   ,
	columns  : Addr<Columns> ,
	addr     : Addr<Self>    ,
}


impl Column
{
	pub fn new( parent: HtmlElement, addr: Addr<Self>, columns: Addr<Columns> ) -> Self
	{
		let container: HtmlElement = document().create_element( "div" ).expect_throw( "create div" ).unchecked_into();
		container.set_class_name( "column" );

		Self { parent, container, addr, columns }
	}


	pub fn logview( &self ) -> Option<HtmlElement>
	{
		self.container.query_selector( ".logview" ).expect_throw( "query_selector" ).map( |e| e.unchecked_into() )
	}


	async fn on_filter( self, mut evts: impl Stream< Item=Event > + Unpin )
	{
		info!( "in on_filter" );

		while let Some(_) = evts.next().await
		{
			if self.logview().is_some()
			{
				debug!( "filtering" );
				self.filter_text()
			}
		}
	}


	fn filter_text( &self )
	{

		let filter: HtmlInputElement = self.container.query_selector( ".filter-input" ).expect_throw( "select filter-input" ).expect_throw( "find fiter-input" ).unchecked_into();

		let filter = filter.value();

		let children = self.logview().expect_throw( "have logview" ).children();

		let mut i = 0;


		while i < children.length()
		{
			let p: HtmlElement = children.item( i ).expect_throw( "get p" ).unchecked_into();
			let text = p.text_content().expect_throw( "text_content" ).to_lowercase();

			if text.contains( &filter.to_lowercase() )
			{
				p.style().set_property( "visibility", "visible" ).expect_throw( "set visibility" );
			}

			else
			{
				p.style().set_property( "visibility", "hidden" ).expect_throw( "set visibility" );
			}

			i += 1;
		}
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
}




pub struct TextBlock
{
	pub block: HtmlElement,
}

unsafe impl Send for TextBlock {}

impl Message for TextBlock { type Return = (); }



impl Handler<TextBlock> for Column
{
	#[async_fn_nosend] fn handle_local( &mut self, msg: TextBlock )
	{
		// if the element exists, remove it
		// add the new one
		// filter the new one.

		if let Some(elem) = self.container.query_selector( ".logview" ).expect_throw( "use query_selector" )
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
	#[async_fn_nosend] fn handle_local( &mut self, _msg: Render )
	{
		let controls: HtmlElement = document().query_selector( ".col-controls" )

			.expect_throw( "find col-controls" )
			.expect_throw( "find col-controls" )
			.clone_node_with_deep( true )
			.expect_throw( "clone filter" )
			.unchecked_into()
		;

		controls.set_class_name( "" );

		let filter = controls.query_selector( ".filter-input" ).expect_throw( "find filter input" ).expect_throw( "find filter input" );

		let filter_evts = EHandler::new( &filter, "input", false );


		self.container.append_child( &controls       ).expect_throw( "append filter" );
		self.parent   .append_child( &self.container ).expect_throw( "append column" );

		spawn_local( Self::on_filter( self.clone(), filter_evts ) );


		let del_col = self.container.query_selector( ".button-close" ).expect_throw( "get close button" ).expect_throw( "get close button" );
		let del_evts = EHandler::new( &del_col, "click", false );

		spawn_local( Column::on_delcol( del_evts, self.addr.clone() ) );
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
	#[async_fn_nosend] fn handle_local( &mut self, msg: DelColumn )
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
