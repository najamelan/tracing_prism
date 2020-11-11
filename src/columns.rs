use crate::{ *, import::*, column::Column };


mod add_column;
mod del_column;
mod current_time;

pub use
{
	add_column   ::* ,
	del_column   ::* ,
	current_time ::* ,
};


#[ derive( Actor ) ]
//
pub struct Columns
{
	children: HashMap<usize, Addr<Column>>,
	container: HtmlElement,
	addr_columns: Addr<Self>,
	addr_control: Addr<Control>,
}



impl Columns
{
	pub fn new( container: HtmlElement, number: usize, addr_columns: Addr<Self>, addr_control: Addr<Control> ) -> Self
	{
		let mut children = HashMap::with_capacity( number+3 );

		for _ in 0..number
		{
			let (addr, mb) = Addr::builder().build();
			let col        = Column::new( container.clone(), addr.clone(), addr_columns.clone(), addr_control.clone() );

			children.insert( addr.id(), addr.clone() );

			spawn_local( async{ mb.start_local( col ).await; } );
		}

		Self
		{
			container,
			children ,
			addr_columns,
			addr_control,
		}
	}


	pub async fn render( &mut self )
	{
		for child in &mut self.children.values_mut()
		{
			child.send( Render ).await.expect_throw( "send Render to column" );
		}
	}
}







