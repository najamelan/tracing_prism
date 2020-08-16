use crate::{ *, import::*, column::Column };


#[ derive( Actor ) ]
//
pub struct Columns
{
	children: Vec<Addr<Column>>,
	container: HtmlElement,
}



impl Columns
{
	pub fn new( container: HtmlElement, number: usize ) -> Self
	{
		let mut children = Vec::with_capacity( number+3 );

		for _ in 0..number
		{
			let col  = Column::new( container.clone() );
			let addr = Addr::builder().start_local( col, &Bindgen ).expect_throw( "start column" );
			children.push( addr );
		}

		Self
		{
			container,
			children ,
		}
	}


	pub async fn render( &mut self )
	{
		for child in &mut self.children
		{
			child.send( Render{} ).await.expect_throw( "send Render to column" );
		}
	}
}




pub struct AddColumn {}

impl Message for AddColumn { type Return = (); }


impl Handler<AddColumn> for Columns
{
	#[async_fn_nosend] fn handle_local( &mut self, _msg: AddColumn )
	{
		let     col  = Column::new( self.container.clone() );
		let mut addr = Addr::builder().start_local( col, &Bindgen ).expect_throw( "start column" );

		addr.send( Render ).await.expect_throw( "send render to column" );

		self.children.push( addr );
	}

	#[async_fn] fn handle( &mut self, _msg: AddColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}




pub struct SetText
{
	pub text: String,
}

impl Message for SetText { type Return = (); }


impl Handler<SetText> for Columns
{
	#[async_fn_nosend] fn handle_local( &mut self, msg: SetText )
	{
		let block = document().create_element( "div" ).expect_throw( "create div tag" );

		for line in msg.text.lines()
		{
			let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();
			p.set_inner_text( line );
			block.append_child( &p ).expect_throw( "append p" );
			block.set_class_name( "logview" );
		}

		for child in &mut self.children
		{
			child.send( TextBlock
			{
				block: block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into()

			}).await.expect_throw( "send textblock to column" );
		}
	}

	#[async_fn] fn handle( &mut self, _msg: SetText )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
