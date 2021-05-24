use crate::{ *, import::* };


pub struct Update
{
	pub block : Option< HtmlElement > ,
	pub filter: Option< Vec<Show>   > ,
	pub format: TextFormat            ,
}

// TODO: use SendWrapper?
//
unsafe impl Send for Update {}


impl Message for Update { type Return = (); }

impl Handler<Update> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: Update )
	{
		// if we get a new text.
		//
		if let Some(block) = &msg.block
		{
			// if the element exists, remove it
			// add the new one
			// filter the new one.

			if let Some(elem) = self.logview()
			{
				// Hopefully leak a bit less memory:
				// https://stackoverflow.com/a/3785323
				// needs some more research.
				//
				elem.set_inner_html( "" );
				elem.remove();
			}

			// set display none if column is collapsed.
			//
			let controls = self.find( ".col-controls" );


			if controls.class_list().contains( "collapsed" )
			{
				block.style().set_property( "display", "none" ).expect_throw( "set display none" );
			}

			if let Some(f) = msg.filter
			{
				self.filter( &block, &f );
			}

			else // correctly apply the odd class.
			{
				let children = block.children();
				let mut odd  = true;

				let mut i = 0;
				let length = children.length();

				while i < length
				{
					let p: HtmlElement = children.item( i ).expect_throw( "get p" ).unchecked_into();
					let class = p.class_list();

					match odd
					{
						true  => class.add_1   ( "odd" ).expect_throw( "add odd class"    ) ,
						false => class.remove_1( "odd" ).expect_throw( "remove odd class" ) ,
					}

					odd ^= true; // .toggle()

					i += 1;
				}
			}


			// This sets the handlers for expanding the log entry on click to show all the fields as well
			// as displaying the timestamp in the lower left corner on mouseover.
			//
			// Both of these don't make sense for plain text logs.
			//
			if let TextFormat::Json = msg.format
			{
				let logview_evts = EHandler::new( &block, "click", true );
				let task         = Column::on_click_entry( logview_evts, self.addr.clone().expect_throw( "have addr" ) );

				self.nursery.spawn_local( task ).expect_throw( "click evts on logview" );


				let logview_evts = EHandler::new( &block, "mouseover", true );
				let task         = Column::on_mouse_over_entry( logview_evts, self.addr.clone().expect_throw( "have addr" ) );

				self.nursery.spawn_local( task ).expect_throw( "click evts on logview" );
			}

			self.container.append_child( &block ).expect_throw( "append div" );
		}



		// if we get new visibility information for lines.
		//
		else if let Some(f) = msg.filter
		{
			if let Some(logview) = self.logview()
			{
				logview.remove();
				self.filter( &logview, &f );
				self.container.append_child( &logview ).expect_throw( "Handler<Update> for Column - append logview");
			}

			else
			{
				// Control shouldn't send us filters unless there is text.
				//
				unreachable!();
			}
		}
	}

	#[async_fn] fn handle( &mut self, _msg: Update )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}

