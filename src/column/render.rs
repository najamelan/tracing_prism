use crate::{ *, import::* };


pub struct Render;

impl Message for Render { type Return = (); }


impl Handler<Render> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: Render )
	{
		let addr = self.addr.as_ref().expect_throw( "Column unwrap addr" ).clone();
		let controls: HtmlElement = document().query_selector( ".col-controls.template" )

			.expect_throw( "find col-controls" )
			.expect_throw( "find col-controls" )
			.clone_node_with_deep( true )
			.expect_throw( "clone filter" )
			.unchecked_into()
		;

		controls.set_class_name( "col-controls" );


		self.container.append_child( &controls       ).expect_throw( "append filter" );
		self.parent   .append_child( &self.container ).expect_throw( "append column" );

		self.control.send( InitColumn( addr.clone() ) )

			.await.expect_throw( "send init column" );

		// Set event listeners on buttons
		// TODO: use drop channel
		//
		let filter      = self.find( ".filter-input" );
		let filter_evts = EHandler::new( &filter, "input", true );
		let task        = Column::on_filter( filter_evts, addr.clone() );

		self.nursery.spawn_local( task ).expect_throw( "Handler<Render> for Column - spawn filter" );


		let del_col  = self.find( ".button-close" );
		let del_evts = EHandler::new( &del_col, "click", true );
		let task     = Column::on_delcol( del_evts, addr.clone() );

		self.nursery.spawn_local( task ).expect_throw( "Handler<Render> for Column - spawn close" );

		self.toggle_button::<Collapse>( ".button-collapse" );

		self.toggle_button::<ToggleTrace>( ".button-trace" );
		self.toggle_button::<ToggleDebug>( ".button-debug" );
		self.toggle_button::<ToggleInfo >( ".button-info"  );
		self.toggle_button::<ToggleWarn >( ".button-warn"  );
		self.toggle_button::<ToggleError>( ".button-error" );
	}


	#[async_fn] fn handle( &mut self, _msg: Render )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
