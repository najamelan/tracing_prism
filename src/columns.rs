use crate::{ *, import::*, column::Column };






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
			let     (addr, mb)    = Addr::builder().build();
			let     col           = Column::new( container.clone(), addr.clone(), addr_columns.clone(), addr_control.clone() );
			let mut addr_control2 = addr_control.clone();

			children.insert( addr.id(), addr.clone() );


			spawn_local( async{ mb.start_local( col ).await; } );

			spawn_local( async move
			{
				addr_control2.send( InitColumn(addr) ).await.expect_throw( "send column to control" );
			});
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
			child.send( Render{} ).await.expect_throw( "send Render to column" );
		}
	}
}




pub struct AddColumn;

impl Message for AddColumn { type Return = (); }


impl Handler<AddColumn> for Columns
{
	#[async_fn_local] fn handle_local( &mut self, _msg: AddColumn )
	{
		let (mut addr, mb) = Addr::builder().build();
		let col            = Column::new( self.container.clone(), addr.clone(), self.addr_columns.clone(), self.addr_control.clone() );

		spawn_local( async{ mb.start_local( col ).await; } );

		addr.send( Render ).await.expect_throw( "send render to column" );

		self.children.insert( addr.id(), addr );
	}

	#[async_fn] fn handle( &mut self, _msg: AddColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}






impl Handler<DelColumn> for Columns
{
	#[async_fn_local] fn handle_local( &mut self, msg: DelColumn )
	{
		self.children.remove( &msg.id );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
