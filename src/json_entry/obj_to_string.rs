use serde_json::Value;
use std::fmt;


/// Turns a json_serde::Value into a string without quotes around fields and values for readability.
//  TODO: Probably need to quote if the text contains =, ", spaces, comma, } ]
pub fn val_to_string( obj: &Value, s: &mut impl fmt::Write ) -> fmt::Result
{
	match obj
	{
		Value::Null      => write!( s, "null"  )?,
		Value::Bool  (b) => write!( s, "{}", b )?,
		Value::Number(n) => write!( s, "{}", n )?,
		Value::String(x) => write!( s, "{}", x )?,

		Value::Array(a) if !a.is_empty()  =>
		{
			write!( s, "[ " )?;

			let mut iter = a.iter();

			val_to_string( iter.next().unwrap(), s )?;

			for elem in iter
			{
				write!( s, ", " )?;
				val_to_string( elem, s )?;
			}

			write!( s, " ]" )?;
		}


		// We know it has at least one key:value pair.
		//
		Value::Object(o) if !o.is_empty() =>
		{
			let mut name = false;

			// This is like the name of the span. We will treat this special and print something like:
			// span_name={ prop1=val1, ... }
			//
			if let Some(v) = o.get( "name" )
			{
				name = true;

				val_to_string( v, s )?;
			}


			// We no longer want to deal with the name property, but since we are working
			// from a read-only object, it's still in here.
			//
			// If there was a name, there must be at least one more property in order to
			// keep processing.
			//
			if !name || o.len() > 1
			{
				if name { write!( s, "={{ " )?; }
				else    { write!( s, "{{ "  )?; }


				// Since we want to separate with comma's, unroll the first element so we
				// don't precede it with a comma.
				//
				let mut iter = o.iter();
				let     elem = iter.next().unwrap();

				if elem.0 != "name"
				{
					// tracing subscriber seems to have code for this, but somehow it
					// doesn't run on json output.
					//
					let key: &str = if   elem.0.starts_with( "r#" ) { &elem.0[2..] }
					                else { elem.0 }
					;


					write!( s, "{}=", key )?;

					val_to_string( elem.1, s )?;
				}


				for elem in iter
				{
					if elem.0 == "name" { continue; }


					let key: &str = if elem.0.starts_with( "r#" ) { &elem.0[2..] }
					                else { elem.0 }
					;

					write!( s, ", {}=", key )?;
					val_to_string( elem.1, s )?;
				}

				write!( s, " }}" )?;
			}
		}

		_ => {}
	};

	Ok(())
}
