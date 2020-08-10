use crate::{ import::*, column::Column };


pub struct Columns
{
	children: Vec<Column>,
	container: HtmlElement,
}


impl Columns
{
	pub fn new( container: HtmlElement, number: usize ) -> Self
	{
		let mut children = Vec::with_capacity( number );

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
}
