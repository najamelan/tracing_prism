#![ allow( unused_imports ) ]


mod column;
mod columns;
mod e_handler;

mod import
{
	pub use
	{
		log             :: { *                              } ,
		web_sys         :: { *, console::log_1 as dbg       } ,
		wasm_bindgen    :: { JsCast, UnwrapThrowExt         } ,
		thespis         :: { *                              } ,
		thespis_impl    :: { *                              } ,
		async_executors :: { *                              } ,
		std             :: { marker::PhantomData, rc::Rc    } ,
		gloo_events     :: { *                              } ,
		futures         :: { Stream, StreamExt, channel::{ mpsc::{ unbounded, UnboundedReceiver, UnboundedSender } } } ,
		std             :: { task::*, pin::Pin, panic                        } ,
		wasm_bindgen_futures :: { spawn_local, JsFuture                      } ,

	};
}

use
{
	column    :: { * } ,
	columns   :: { * } ,
	e_handler :: { * } ,
};


use import::*;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub async fn main()
{
	console_log::init_with_level( log::Level::Trace ).expect_throw( "initialize logger" );

	// // start new actor
	// //
	// let     a    = MyActor { count: 10, phantom: PhantomData };
	// let mut exec = Bindgen{};
	// let mut addr = Addr::builder().start( MyActor, &exec ).expect( "create addres for MyActor" );

	// // send message and get future for result
	// //
	// let res = addr.call( Ping(5) ).await.expect( "Send Ping" );

	let window   = web_sys::window  ().expect_throw( "no global `window` exists"        );
	let document = window  .document().expect_throw( "should have a document on window" );
	// let body     = document.body    ().expect_throw( "document should have a body"      );
	//
	let upload = get_id( "upload" );

	let file_evts   = EHandler::new( &upload, "change", false );


	let column_cont: HtmlElement = document.get_element_by_id( "columns" ).expect_throw( "doc should have columns element" ).unchecked_into();

	let columns = Columns::new( column_cont, 3 );

	columns.render();

	spawn_local( on_upload( file_evts, columns ) );
	// // Manufacture the element we're gonna append
	// //
	// let val = document.create_element( "div" ).expect( "Failed to create div" );

	// val.set_inner_html( &format!( "The pong value is: {}", res ) );

	// body.append_child( &val ).expect( "Coundn't append child" );
}




pub fn document() -> Document
{
	let window = web_sys::window().expect_throw( "no global `window` exists");

	window.document().expect_throw( "should have a document on window" )
}



fn get_id( id: &str ) -> HtmlElement
{
	document().get_element_by_id( id ).expect_throw( &format!( "find {}", id ) ).unchecked_into()
}


async fn on_upload
(
	mut evts: impl Stream< Item=Event > + Unpin ,
	columns: Columns,
)
{
	debug!( "in on_upload" );

	let upload: HtmlInputElement = get_id( "upload" ).unchecked_into();

	while let Some(_) = evts.next().await
	{
		let file_list = upload.files().expect_throw( "get filelist" );
		let file = file_list.get( 0 ).expect_throw( "get first file" );

		let text = JsFuture::from( file.text() ).await.expect_throw( "file upload complete" ).as_string().expect_throw( "string content" );

		columns.set_text( text );
	};
}
