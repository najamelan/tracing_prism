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

		let log: HtmlElement = document().create_element( "pre" ).expect_throw( "create pre" ).unchecked_into();

		self.container.append_child( &filter ).expect_throw( "append filter" );
		self.container.append_child( &log    ).expect_throw( "append log"    );

		self.parent.append_child( &self.container ).expect_throw( "append column" );
	}
}
