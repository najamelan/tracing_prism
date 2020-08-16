use crate::{ *, import::*, column::Column };


pub struct Columns
{
	children: Vec<Column>,
	container: HtmlElement,
}


impl Columns
{
	pub fn new( container: HtmlElement, number: usize ) -> Self
	{
		let mut children = Vec::with_capacity( number+3 );

		for _ in 0..number
		{
			children.push( Column::new( container.clone() ) );
		}

		Self
		{
			container,
			children ,
		}
	}


	pub fn render( &self )
	{
		for child in &self.children
		{
			child.render();
		}
	}


	pub fn set_text( &self, text: String )
	{
		let block = document().create_element( "div" ).expect_throw( "create div tag" );

		for line in text.lines()
		{
			let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();
			p.set_inner_text( line );
			block.append_child( &p ).expect_throw( "append p" );
			block.set_class_name( "logview" );
		}

		for child in &self.children
		{
			child.set_text( block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into() );
		}
	}


	pub fn add_column( &mut self )
	{
		self.children.push( Column::new( self.container.clone() ) );
		self.children.last().unwrap().render();
	}
}
