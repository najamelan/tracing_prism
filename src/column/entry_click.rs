use crate::{ *, import::*, ToggleEntry };



#[ derive( Debug, Clone ) ]
//
pub struct EntryClick
{
	pub evt: SendWrapper<Event>
}


impl Message for EntryClick { type Return = (); }


impl Handler<EntryClick> for Column
{
	#[async_fn_local] fn handle_local( &mut self, msg: EntryClick )
	{
		let mut target: HtmlElement = msg.evt.target().expect_throw( "event has target" ).dyn_into().expect( "HtmlElement" );

		// We can click between entries and thus end up on the logview. In that case disregard the click.
		//
		if target.class_list().contains( "logview" ) { return; }


		// target could be a descendant of entry, walk the tree.
		//
		if !target.class_list().contains( "entry" )
		{
			while let Some( element ) = target.parent_node()
			{
				let element: HtmlElement = element.dyn_into().expect( "HtmlElement" );

				if target.class_list().contains( "logview" ) { return; }

				if element.class_list().contains( "entry" )
				{
					target = element;

					break;
				}

				target = element
			}
		}


		let id = target.id();

		self.control.send( ToggleEntry{ id } ).await.expect_throw( "send" );
	}

	#[async_fn] fn handle( &mut self, _msg: EntryClick )
	{
		unreachable!( "This actor is !Send and cannot be spawned on a threadpool" );
	}
}
