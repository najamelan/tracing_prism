#![ allow(clippy::suspicious_else_formatting) ]

mod column;
mod columns;
mod control;
mod e_handler;
mod entry;
mod json_entry;
mod plain_entry;
mod util;

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
		async_nursery   :: { *                              } ,
		std             :: { marker::PhantomData, rc::Rc    } ,
		gloo_events     :: { *                              } ,
		futures         :: { Stream, StreamExt, channel::{ mpsc::{ unbounded, UnboundedReceiver, UnboundedSender } } } ,
		futures         :: { task::LocalSpawnExt } ,
		std             :: { task::*, pin::Pin, panic, collections::HashMap, sync::Arc, convert::TryInto  } ,
		wasm_bindgen_futures :: { spawn_local, JsFuture                      } ,
		regex           :: { Regex                          } ,
		send_wrapper    :: { SendWrapper                    } ,
	};
}


use
{
	column     :: { * } ,
	columns    :: { * } ,
	control    :: { * } ,
	e_handler  :: { * } ,
	entry      :: { * } ,
	json_entry :: { * } ,
	plain_entry:: { * } ,
	util       :: { * } ,
};


use import::*;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub async fn main()
{
	wasm_logger::init( wasm_logger::Config::new( log::Level::Trace ) );

	let upload = get_id( "upload" );

	let file_evts = EHandler::new( &upload, "change", true );

	let add_col  = get_id( "add-column" );
	let add_evts = EHandler::new( &add_col, "click", true );


	let column_cont  = get_id( "columns" );
	let addr_control = Addr::builder().start_local( Control::new(), &Bindgen ).expect_throw( "spawn control" );

	Err( "test panicking in wasm" ).expect_throw( "BOOM" );

	let (addr_columns, mb_columns) = Addr::builder().build();
	let mut columns = Columns::new( column_cont, 3, addr_columns.clone(), addr_control.clone() );
	columns.render().await;

	spawn_local( async{ mb_columns.start_local( columns ).await; } );

	spawn_local( on_upload( file_evts, addr_control ) );
	spawn_local( on_addcol( add_evts , addr_columns ) );
}




async fn on_upload
(
	mut evts: impl Stream< Item=Event > + Unpin ,
	mut control: Addr<Control>,
)
{
	let upload: HtmlInputElement = get_id( "upload" ).unchecked_into();

	while evts.next().await.is_some()
	{
		let file_list = upload.files().expect_throw( "get filelist" );
		let file      = file_list.get( 0 ).expect_throw( "get first file" );

		let text = JsFuture::from( file.text() ).await.expect_throw( "file upload complete" ).as_string().expect_throw( "string content" );

		control.send( SetText{ text } ).await.expect_throw( "send settext" );
	};
}


async fn on_addcol
(
	evts: impl Stream< Item=Event > + Unpin ,
	columns: Addr<Columns>,
)
{
	evts

		.map( |_| Ok( AddColumn ) )
		.forward( columns ).await
		.expect_throw( "send addcol" )
	;
}
