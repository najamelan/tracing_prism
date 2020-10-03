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
pub struct Entry
{
	pub lvl: LogLevel ,
	pub txt: String   ,
}


impl Entry
{
	pub fn new( txt: String ) -> Self
	{
		let lvl =

			     if txt.contains( " TRACE " ) { LogLevel::Trace   }
			else if txt.contains( " DEBUG " ) { LogLevel::Debug   }
			else if txt.contains( " INFO "  ) { LogLevel::Info    }
			else if txt.contains( " WARN "  ) { LogLevel::Warn    }
			else if txt.contains( " ERROR " ) { LogLevel::Error   }
			else                              { LogLevel::Unknown }
		;

		Self { lvl, txt }
	}


	/// Should this line be shown for the given filter?
	//
	pub fn show( &self, filter: &Filter ) -> bool
	{
		let mut show = true;


		if  !filter.txt.is_empty()  &&  !self.txt.to_lowercase().contains( &filter.txt )
		{
			show = false;
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
}
