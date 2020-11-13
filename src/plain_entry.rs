use crate::{ *, import::* };


pub enum LogLevel
{
	Trace   ,
	Debug   ,
	Info    ,
	Warn    ,
	Error   ,
	Unknown ,
}


/// Represents one line in the log.
///
/// Functionality:
///
/// - detect which log level a line is
/// - determine whether this line is to be shown based on a given filter
//
pub struct PlainEntry
{
	pub lvl  : LogLevel ,
	pub txt  : String   ,
}


impl PlainEntry
{
	pub fn new( txt: String ) -> Self
	{
		let lvl =

			     if txt.contains( "TRACE " ) { LogLevel::Trace   }
			else if txt.contains( "DEBUG " ) { LogLevel::Debug   }
			else if txt.contains( "INFO "  ) { LogLevel::Info    }
			else if txt.contains( "WARN "  ) { LogLevel::Warn    }
			else if txt.contains( "ERROR " ) { LogLevel::Error   }
			else                             { LogLevel::Unknown }
		;

		Self { lvl, txt }
	}


	/// Should this line be shown for the given filter?
	//
	pub fn matches( &self, filter: &Filter ) -> bool
	{
		let mut show = true;


		if let Some(regex) = &filter.regex
		{
			if !regex.is_match( &self.txt )
			{
				show = false;
			}
		}


		match self.lvl
		{
			LogLevel::Trace   => if !filter.trace { show = false } ,
			LogLevel::Debug   => if !filter.debug { show = false } ,
			LogLevel::Info    => if !filter.info  { show = false } ,
			LogLevel::Warn    => if !filter.warn  { show = false } ,
			LogLevel::Error   => if !filter.error { show = false } ,
			LogLevel::Unknown => {} // always show
		}

		show
	}


	pub fn html( &self ) -> HtmlElement
	{
		let div   : HtmlElement = document().create_element( "div"   ).expect_throw( "create div tag"   ).unchecked_into();
		let p: HtmlElement = document().create_element( "p" ).expect_throw( "create p tag" ).unchecked_into();

		let class = match self.lvl
		{
			LogLevel::Trace   => "trace"          ,
			LogLevel::Debug   => "debug"          ,
			LogLevel::Info    => "info"           ,
			LogLevel::Warn    => "warn"           ,
			LogLevel::Error   => "error"          ,
			LogLevel::Unknown => "unknown_loglvl" ,
		};

		div.class_list().add_1( "entry" ).expect_throw( "add entry to div"  );
		div.class_list().add_1( class   ).expect_throw( "add class to div"  );

		p.set_inner_text( &self.txt );
		div.append_child( &p        ).expect_throw( "append_child to div" );

		div
	}
}
