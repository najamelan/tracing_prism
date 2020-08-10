use crate::{ import::*, * };

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
		let filter: HtmlElement = get_id( "filter-base" ).clone_node_with_deep( true ).expect( "clone filter" ).unchecked_into();
		filter.set_class_name( "filter-input" );

		self.container.append_child( &filter ).expect_throw( "append filter" );

		self.parent.append_child( &self.container ).expect_throw( "append column" );
	}


	pub fn set_text( &self, div: HtmlElement )
	{
		// if the element exists, remove it
		// add the new one

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
}
