use crate::{ *, import::* };
use serde_json::Value;

#[ derive( Copy, Clone, Debug, Eq, PartialEq ) ]
//
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
	pub lvl      : LogLevel       ,
	pub msg      : String         ,
	pub target   : String         ,
}


impl JsonEntry
{
	pub fn new( txt: String ) -> Result<Self, serde_json::Error>
	{
		let mut value: Value = serde_json::from_str( &txt )?;

		let map    = value.as_object_mut().expect( "json to be object" );
		let fields = map.remove( "fields" );

		if let Some( Value::Object(fields) ) = fields
		{
			for (key, value) in fields.into_iter()
			{
				map.insert( key, value );
			}
		}


		// Add the line number directly to the file path:
		//
		if let Some(Value::Number(line)) = map.remove( "log.line" )
		{
			let line = line.as_u64().expect_throw( "line number to be u64" );
			let file = map.get_mut( "log.file" );

			if let Some( Value::String(file) ) = file
			{
				use std::fmt::Write;

				write!( file, ":{}", line ).expect_throw( "write into String" );
			}
		}


		// Store the level separately:
		//
		let lvl = match map.get( "level" )
		{
			Some( Value::String(s) ) if s == "TRACE" => LogLevel::Trace   ,
			Some( Value::String(s) ) if s == "DEBUG" => LogLevel::Debug   ,
			Some( Value::String(s) ) if s == "INFO"  => LogLevel::Info    ,
			Some( Value::String(s) ) if s == "WARN"  => LogLevel::Warn    ,
			Some( Value::String(s) ) if s == "ERROR" => LogLevel::Error   ,
			_                                        => LogLevel::Unknown ,
		};


		// These are duplicate entries
		// with "target".
		//
		map.remove( "log.target"      );
		map.remove( "log.module_path" );

		// We don't print level, but represent it with colors.
		//
		map.remove( "level" );


		let msg = match map.remove( "message" )
		{
			Some(Value::String(s)) => format!( " ~ {}", s ),
			_ => panic!( "every log entry to have a message" ),
		};

		let target = match map.remove( "target" )
		{
			Some(Value::String(s)) => s,
			_ => panic!( "every log entry to have a target" ),
		};



		Ok( Self { value, value_txt: None, lvl, msg, target } )
	}


	pub fn get_value<'a>( value: &'a Value, key: &str ) -> Option<&'a Value>
	{
		let map = value.as_object().expect( "json to be object" );

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
		self.lvl.clone()
	}


	pub fn keys( &self ) -> Vec<&str>
	{
		let mut out = Vec::new();
		let map = self.value.as_object().expect( "json to be object" );

		for key in map.keys()
		{
			out.push( key.as_str().trim() );
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
				let mut value_txt = self.values().join( " " );
				value_txt.push_str( &self.msg );
				self.value_txt = Some( value_txt );

				self.value_txt.as_ref().unwrap()
			}

			Some( value_txt ) => &value_txt,
		};


		if let Some(regex) = &filter.regex
		{
			if !regex.is_match( &value_txt )
			{
				show = false;
			}
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
		let div   : HtmlElement = document().create_element( "div"   ).expect_throw( "create div tag"   ).unchecked_into();
		let p     : HtmlElement = document().create_element( "p"     ).expect_throw( "create p tag"     ).unchecked_into();
		let target: HtmlElement = document().create_element( "span"  ).expect_throw( "create span tag"  ).unchecked_into();
		let msg   : HtmlElement = document().create_element( "span"  ).expect_throw( "create span tag"  ).unchecked_into();
		let t     : HtmlElement = document().create_element( "table" ).expect_throw( "create table tag" ).unchecked_into();

		let class = match self.lvl()
		{
			LogLevel::Trace   => "trace"          ,
			LogLevel::Debug   => "debug"          ,
			LogLevel::Info    => "info"           ,
			LogLevel::Warn    => "warn"           ,
			LogLevel::Error   => "error"          ,
			LogLevel::Unknown => "unknown_loglvl" ,
		};

		div   .class_list().add_1( "entry"   ).expect_throw( "add entry to div"  );
		div   .class_list().add_1( class     ).expect_throw( "add class to div"  );
		p     .class_list().add_1( class     ).expect_throw( "add class to p"    );
		target.class_list().add_1( "target"  ).expect_throw( "add class to span" );
		msg   .class_list().add_1( "message" ).expect_throw( "add class to span" );

		// TODO: we really shouldn't have to put the class on the table, but somehow some CSS didn't stick.
		//
		t  .class_list().add_1( class          ).expect_throw( "add class to t"   );
		t  .class_list().add_1( "display_none" ).expect_throw( "add class to t"   );

		let colgroup: HtmlElement = document().create_element( "colgroup" ).expect_throw( "create colgroup tag" ).unchecked_into();
		let col1    : HtmlElement = document().create_element( "col"      ).expect_throw( "create col      tag" ).unchecked_into();
		let col2    : HtmlElement = document().create_element( "col"      ).expect_throw( "create col      tag" ).unchecked_into();
		let col3    : HtmlElement = document().create_element( "col"      ).expect_throw( "create col      tag" ).unchecked_into();

		col1.class_list().add_1( "field-keys"       ).expect_throw( "add field-keys class to col" );
		col2.class_list().add_1( "field-separators" ).expect_throw( "add field-keys class to col" );
		col3.class_list().add_1( "field-values"     ).expect_throw( "add field-keys class to col" );

		colgroup.append_child( &col1 ).expect_throw( "append col1" );
		colgroup.append_child( &col2 ).expect_throw( "append col2" );
		colgroup.append_child( &col3 ).expect_throw( "append col3" );

		t.append_child( &colgroup ).expect_throw( "append colgroup" );


		for key in self.keys()
		{
			let tr : HtmlElement = document().create_element( "tr" ).expect_throw( "create tr tag" ).unchecked_into();
			let td : HtmlElement = document().create_element( "td" ).expect_throw( "create td tag" ).unchecked_into();
			let td2: HtmlElement = document().create_element( "td" ).expect_throw( "create td tag" ).unchecked_into();
			let td3: HtmlElement = document().create_element( "td" ).expect_throw( "create td tag" ).unchecked_into();

			td.set_inner_text( key );
			td.class_list().add_1( "field-key" ).expect_throw( "add field-key class" );

			td2.set_inner_text( ": " );
			td2.class_list().add_1( "field-separator" ).expect_throw( "add field-key class" );

			let value = self.get(key).expect_throw( "keys to exist" );

			let s = match &value
			{
				// spaces after the colon are non-breaking spaces.
				//
				Value::Null       => "null".to_string()                                        ,
				Value::String (s) => format!( "{}"  , s )                                      ,
				Value::Number (n) => format!( "{}"  , n )                                      ,
				Value::Bool   (b) => format!( "{}"  , b )                                      ,
				Value::Array  (a) => serde_json::to_string(a).expect_throw( "stringify json" ) ,
				Value::Object (o) => serde_json::to_string(o).expect_throw( "stringify json" ) ,
			};

			if key == "timestamp"
			{
				div.set_attribute( "data-time", &s ).expect_throw( "set data-time attribute" );
			}

			td3.set_inner_text( &s );
			td3.class_list().add_1( "field-value" ).expect_throw( "add field-key class" );

			tr.append_child( &td  ).expect_throw( "append_child to tr" );
			tr.append_child( &td2 ).expect_throw( "append_child to tr" );
			tr.append_child( &td3 ).expect_throw( "append_child to tr" );

			t.append_child( &tr ).expect_throw( "append_child to table" );
		}

		msg   .set_inner_text( &self.msg    );
		target.set_inner_text( &self.target );

		p  .append_child( &target ).expect_throw( "append_child to p"   );
		p  .append_child( &msg    ).expect_throw( "append_child to p"   );
		div.append_child( &p      ).expect_throw( "append_child to div" );
		div.append_child( &t      ).expect_throw( "append_child to div" );

		div
	}
}
