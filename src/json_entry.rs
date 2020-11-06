use crate::{ *, import::* };
use serde_json::Value;

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
pub struct JsonEntry
{
	pub value    : Value          ,
	pub value_txt: Option<String> ,
}


impl JsonEntry
{
	pub fn new( txt: String ) -> Result<Self, serde_json::Error>
	{
		let value: Value = serde_json::from_str( &txt )?;

		Ok( Self { value, value_txt: None } )
	}


	pub fn get_value<'a>( value: &'a Value, key: &str ) -> Option<&'a Value>
	{
		let map = value.as_object().expect( "json to be object" );

		// check the fields sub-array first.
		//
		if let Some(o) = map.get( "fields" )
		{
			if let Some( s ) = Self::get_value( o, key )
			{
				return Some(s);
			}
		}

		// check the top level
		//
		map.get( key )
	}


	pub fn get( &self, key: &str ) -> Option<&Value>
	{
		Self::get_value( &self.value, key )
	}


	pub fn lvl( &self ) -> LogLevel
	{
		match self.value.get( "level" )
		{
			Some( Value::String(s) ) if s == "TRACE" => LogLevel::Trace   ,
			Some( Value::String(s) ) if s == "DEBUG" => LogLevel::Debug   ,
			Some( Value::String(s) ) if s == "INFO"  => LogLevel::Info    ,
			Some( Value::String(s) ) if s == "WARN"  => LogLevel::Warn    ,
			Some( Value::String(s) ) if s == "ERROR" => LogLevel::Error   ,
			_                                        => LogLevel::Unknown ,
		}
	}


	pub fn keys( &self ) -> Vec<&str>
	{
		let mut out = Vec::new();
		let map = self.value.as_object().expect( "json to be object" );

		for key in map.keys()
		{
			if key != "fields"
			{
				out.push( key.as_str().trim() );
			}
		}

		if let Some( Value::Object( fields ) ) = self.value.get( "fields" )
		{
			for key in fields.keys()
			{
				out.push( key.as_str().trim() );
			}
		}

		out.sort();
		out.dedup();
		out
	}


	pub fn values( &self ) -> Vec<String>
	{
		let mut out = Vec::new();

		for key in self.keys()
		{
			let value = self.get(key).expect_throw( "keys to exist" );
			let s     = serde_json::to_string( &value ).expect_throw( "serialize serde_json::Value" );
			out.push( s.trim().to_string() );
		}

		out.sort();
		out.dedup();
		out
	}


	/// Should this line be shown for the given filter?
	//
	pub fn matches( &mut self, filter: &Filter ) -> bool
	{
		let mut show = true;

		let value_txt = match &self.value_txt
		{
			None =>
			{
				self.value_txt = Some( self.values().join( " " ) );
				self.value_txt.as_ref().unwrap()
			}

			Some( value_txt ) => &value_txt,
		};


		if  !filter.txt.is_empty()
		&&  UniCase::new( value_txt ) != UniCase::new( &filter.txt )
		{
			show = false;
		}


		match self.lvl()
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
		let t: HtmlElement = document().create_element( "table" ).expect_throw( "create table tag" ).unchecked_into();

		let class = match self.lvl()
		{
			LogLevel::Trace   => "trace"          ,
			LogLevel::Debug   => "debug"          ,
			LogLevel::Info    => "info"           ,
			LogLevel::Warn    => "warn"           ,
			LogLevel::Error   => "error"          ,
			LogLevel::Unknown => "unknown_loglvl" ,
		};

		t.class_list().add_1( class ).expect_throw( "add class to table" );

		for key in self.keys()
		{
			debug!( "{}", key );

			let tr : HtmlElement = document().create_element( "tr" ).expect_throw( "create tr tag" ).unchecked_into();
			let td : HtmlElement = document().create_element( "td" ).expect_throw( "create td tag" ).unchecked_into();
			let td2: HtmlElement = document().create_element( "td" ).expect_throw( "create td tag" ).unchecked_into();

			td .set_inner_text( key );


			let value = self.get(key).expect_throw( "keys to exist" );
			let s     = serde_json::to_string( &value ).expect_throw( "serialize serde_json::Value" );

			td2.set_inner_text( &s );

			tr.append_child( &td  ).expect_throw( "append_child to tr" );
			tr.append_child( &td2 ).expect_throw( "append_child to tr" );

			t.append_child( &tr ).expect_throw( "append_child to table" );
		}

		t
	}
}
