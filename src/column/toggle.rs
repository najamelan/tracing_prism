use crate::{ *, import::* };



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleTrace;

impl Message for ToggleTrace { type Return = (); }


impl Handler<ToggleTrace> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleTrace )
	{
		self.filter.trace = !self.filter.trace;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-trace" );

		if self.filter.trace
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-trace" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-trace" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleTrace )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleDebug;

impl Message for ToggleDebug { type Return = (); }


impl Handler<ToggleDebug> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleDebug )
	{
		self.filter.debug = !self.filter.debug;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-debug" );

		if self.filter.debug
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-debug" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-debug" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleDebug )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleInfo;

impl Message for ToggleInfo { type Return = (); }


impl Handler<ToggleInfo> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleInfo )
	{
		self.filter.info = !self.filter.info;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-info" );

		if self.filter.info
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-info" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-info" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleInfo )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleWarn;

impl Message for ToggleWarn { type Return = (); }


impl Handler<ToggleWarn> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleWarn )
	{
		self.filter.warn = !self.filter.warn;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-warn" );

		if self.filter.warn
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-warn" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-warn" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleWarn )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}



#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct ToggleError;

impl Message for ToggleError { type Return = (); }


impl Handler<ToggleError> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: ToggleError )
	{
		self.filter.error = !self.filter.error;

		self.control.send( self.filter.clone() ).await.expect_throw( "update filter" );


		let button = self.find( ".button-error" );

		if self.filter.error
		{
			button.class_list().remove_1( "hide" ).expect_throw( "remove hide from button-error" );
		}

		else
		{
			button.class_list().add_1( "hide" ).expect_throw( "add hide to button-error" );
		}
	}


	#[async_fn] fn handle( &mut self, _msg: ToggleError )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
