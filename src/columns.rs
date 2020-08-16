use crate::{ *, import::*, column::Column };


#[ derive( Actor ) ]
//
pub struct Columns
{
	children: HashMap<usize, Addr<Column>>,
	container: HtmlElement,
	last_text: Option<HtmlElement>,
}



impl Columns
{
	pub fn new( container: HtmlElement, number: usize ) -> Self
	{
		let mut children = HashMap::with_capacity( number+3 );

		for _ in 0..number
		{
			let (addr, mb) = Addr::builder().build();
			let col        = Column::new( container.clone(), addr.id() );

			Bindgen.spawn_local( async{ mb.start_local( col ).await; } ).expect_throw( "start column" );

			children.insert( addr.id(), addr );
		}

		Self
		{
			container,
			children ,
			last_text: None,
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
	#[async_fn_nosend] fn handle_local( &mut self, _msg: AddColumn )
	{
		let (mut addr, mb) = Addr::builder().build();
		let col        = Column::new( self.container.clone(), addr.id() );

		Bindgen.spawn_local( async{ mb.start_local( col ).await; } ).expect_throw( "start column" );

		addr.send( Render ).await.expect_throw( "send render to column" );

		if let Some( block ) = &self.last_text
		{
			addr.send( TextBlock
			{
				block: block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into()

			}).await.expect_throw( "send textblock to column" );
		}

		self.children.insert( addr.id(), addr );
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
		let block: HtmlElement = document().create_element( "div" ).expect_throw( "create div tag" ).unchecked_into();

		for line in msg.text.lines()
		{
			let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();
			p.set_inner_text( line );
			block.append_child( &p ).expect_throw( "append p" );
			block.set_class_name( "logview" );
		}

		for child in &mut self.children.values_mut()
		{
			child.send( TextBlock
			{
				block: block.clone_node_with_deep( true ).expect_throw( "clone text" ).unchecked_into()

			}).await.expect_throw( "send textblock to column" );
		}

		self.last_text = Some( block );
	}

	#[async_fn] fn handle( &mut self, _msg: SetText )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
