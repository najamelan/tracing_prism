use crate::{ *, import::* };


#[ derive( Debug, Default, Copy, Clone ) ]
//
pub struct Collapse;

impl Message for Collapse { type Return = (); }


impl Handler<Collapse> for Column
{
	#[async_fn_local] fn handle_local( &mut self, _msg: Collapse )
	{
		// turn controls sideways
		//
		let controls = self.find( ".col-controls" );


		if self.container.class_list().contains( "collapsed" )
		{
			self.container.class_list().remove_1( "collapsed" ).expect_throw( "remove collapsed class" );

			// set with of column to height of controls
			//
			self.container.style().set_property( "width", "auto" ).expect_throw( "set width" );


			if let Some( logview ) = self.logview()
			{
				logview.style().set_property( "display", "block" ).expect_throw( "set display block" );
			}
		}

		else
		{
			self.container.class_list().add_1( "collapsed" ).expect_throw( "add collapsed class" );

			// set with of column to height of controls
			//
			let width = controls.get_bounding_client_rect().width();
			self.container.style().set_property( "width", &format!( "{}", width ) ).expect_throw( "set width" );


			if let Some( logview ) = self.logview()
			{
				logview.style().set_property( "display", "none" ).expect_throw( "set display none" );
			}
		}
	}


	#[async_fn] fn handle( &mut self, _msg: Collapse )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}

