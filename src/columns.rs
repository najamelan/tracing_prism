use crate::{ *, import::*, column::Column };



pub enum LogLevel
{
	Trace,
	Debug,
	Info,
	Warn,
	Error,
	Unknown,
}


#[ derive( Actor ) ]
//
pub struct Columns
{
	children: HashMap<usize, Addr<Column>>,
	container: HtmlElement,
	last_text: Option<HtmlElement>,
	addr_columns: Addr<Self>,
}



impl Columns
{
	pub fn new( container: HtmlElement, number: usize, addr_columns: Addr<Self> ) -> Self
	{
		let mut children = HashMap::with_capacity( number+3 );

		for _ in 0..number
		{
			let (addr, mb) = Addr::builder().build();
			let col        = Column::new( container.clone(), addr.clone(), addr_columns.clone() );

			spawn_local( async{ mb.start_local( col ).await; } );

			children.insert( addr.id(), addr );
		}

		Self
		{
			container,
			children ,
			last_text: None,
			addr_columns,
		}
	}


	pub async fn render( &mut self )
	{
		for child in &mut self.children.values_mut()
		{
			child.send( Render{} ).await.expect_throw( "send Render to column" );
		}
	}


	pub fn loglevel( line: &str ) -> LogLevel
	{
		     if line.contains( " TRACE " ) { LogLevel::Trace   }
		else if line.contains( " DEBUG " ) { LogLevel::Debug   }
		else if line.contains( " INFO "  ) { LogLevel::Info    }
		else if line.contains( " WARN "  ) { LogLevel::Warn    }
		else if line.contains( " ERROR " ) { LogLevel::Error   }
		else                               { LogLevel::Unknown }
	}
}




pub struct AddColumn;

impl Message for AddColumn { type Return = (); }


impl Handler<AddColumn> for Columns
{
	#[async_fn_nosend] fn handle_local( &mut self, _msg: AddColumn )
	{
		let (mut addr, mb) = Addr::builder().build();
		let col        = Column::new( self.container.clone(), addr.clone(), self.addr_columns.clone() );

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
		block.set_class_name( "logview" );


		for line in msg.text.lines()
		{
			let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();

			let class = match Self::loglevel( &line )
			{
				LogLevel::Trace   => "trace"          ,
				LogLevel::Debug   => "debug"          ,
				LogLevel::Info    => "info"           ,
				LogLevel::Warn    => "warn"           ,
				LogLevel::Error   => "error"          ,
				LogLevel::Unknown => "unknown_loglvl" ,
			};

			p.class_list().add_1( class ).expect_throw( "add class to p" );
			p.set_inner_text( line );
			block.append_child( &p ).expect_throw( "append p" );
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




impl Handler<DelColumn> for Columns
{
	#[async_fn_nosend] fn handle_local( &mut self, msg: DelColumn )
	{
		self.children.remove( &msg.id );
	}

	#[async_fn] fn handle( &mut self, _msg: DelColumn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
