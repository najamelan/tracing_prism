use crate::{ *, import::*, JsonEntry, PlainEntry };


/// Let's us specify the data type in messages.
//
#[ derive( Debug, Copy, Clone, PartialEq, Eq, Hash ) ]
//
pub enum TextFormat
{
	Plain,
	Json ,
}


pub enum Entry
{
	Plain( PlainEntry ) ,
	Json ( JsonEntry  ) ,
}


impl Entry
{
	pub fn matches( &mut self, filter: &Filter ) -> bool
	{
		match self
		{
			Self::Plain(e) => e.matches(filter),
			Self::Json (e) => e.matches(filter),
		}
	}


	pub fn html( &self ) -> HtmlElement
	{
		match self
		{
			Self::Plain(e) => e.html(),
			Self::Json (e) => e.html(),
		}
	}
}
