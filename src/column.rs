use crate::{ import::*, * };


#[ derive( Clone ) ]
//
pub struct Column
{
	parent: HtmlElement,
	container: HtmlElement,
}


impl Column
{
	pub fn new( parent: HtmlElement ) -> Self
	{
		let container: HtmlElement = document().create_element( "div" ).expect_throw( "create div" ).unchecked_into();
		container.set_class_name( "column" );

		Self { parent, container }
	}


	pub fn render( &self )
	{
		let filter: HtmlElement = get_id( "col-controls" ).clone_node_with_deep( true ).expect( "clone filter" ).unchecked_into();
		filter.set_class_name( "filter-input" );

		self.container.append_child( &filter ).expect_throw( "append filter" );

		self.parent.append_child( &self.container ).expect_throw( "append column" );

		let filter_evts = EHandler::new( &filter, "input", false );

		spawn_local( Self::on_filter( self.clone(), filter_evts ) );
	}


	pub fn set_text( &self, div: HtmlElement )
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

		self.container.append_child( &div ).expect_throw( "append div" );
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
}


