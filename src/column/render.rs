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

		let searchbox: HtmlElement = controls

			.query_selector( ".filter-input" )
			.expect_throw( "find .filter-input" )
			.expect_throw( "find .filter-input" )
			.unchecked_into()
		;

		// This makes it so that tabbing through elements first goes through the filter text input
		// of each column before going to all the buttons for the log levels.
		// We need each column to have an increasing number. Using the addr.id() here is not very
		// robust but it is convenient. We add an arbitrary number so we don't interfere with the
		// form elements in the header. Other elements on the page will not have any tab-index set
		// so they will come after these.
		//
		let t_index = addr.id() +10;
		searchbox.set_tab_index( t_index as i32 );

		self.container.append_child( &controls       ).expect_throw( "append filter" );
		self.parent   .append_child( &self.container ).expect_throw( "append column" );

		self.control.send( InitColumn( addr.clone() ) )

			.await.expect_throw( "send init column" );

		// Set event listeners on buttons
		// TODO: use drop channel?
		//
		let filter      = self.find( ".filter-input" );
		let filter_evts = EHandler::new( &filter, "input", true );
		let task        = Column::on_filter( filter_evts, addr.clone() );

		self.nursery.nurse_local( task ).expect_throw( "Handler<Render> for Column - spawn filter" );


		let del_col  = self.find( ".button-close" );
		let del_evts = EHandler::new( &del_col, "click", true );
		let task     = Column::on_delcol( del_evts, addr.clone() );

		self.nursery.nurse_local( task ).expect_throw( "Handler<Render> for Column - spawn close" );


		// Apparently click is a better event for this than onchange. In any case, we want to
		// react as fast as possible and not wait for the element to lose focus.
		//
		let use_regex  = get_id( "use-regex" );
		let regex_evts = EHandler::new( &use_regex, "click", true );
		let task       = Column::on_use_regex( regex_evts, addr.clone() );

		self.nursery.nurse_local( task ).expect_throw( "Handler<Render> for Column - spawn regex" );


		let case      = get_id( "case" );
		let case_evts = EHandler::new( &case, "click", true );
		let task      = Column::on_case_sensitive( case_evts, addr.clone() );

		self.nursery.nurse_local( task ).expect_throw( "Handler<Render> for Column - spawn case" );


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
