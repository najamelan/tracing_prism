use crate::import::*;

/// Get the document node
//
pub fn document() -> Document
{
	window().document().expect_throw( "should have a document on window" )
}


/// GetElementById
//
pub fn get_id( id: &str ) -> HtmlElement
{
	document().get_element_by_id( id ).expect_throw( &format!( "find {}", id ) ).unchecked_into()
}


/// web_sys::window()
//
pub fn window() -> Window
{
	web_sys::window().expect_throw( "no global `window` exists" )
}
