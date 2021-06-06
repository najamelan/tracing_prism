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

		Value::Array(a) if a.len() > 0  =>
		{
			write!( s, "[ " )?;

			let mut iter = a.iter();

			val_to_string( iter.next().unwrap(), s )?;

			for elem in iter
			{
				write!( s, ", " )?;
				val_to_string( &elem, s )?;
			}

			write!( s, " ]" )?;
		}


		Value::Object(o) if o.len() > 0 =>
		{
			write!( s, "{{ " )?;

			let mut iter = o.iter();
			let     elem = iter.next().unwrap();


			let key: &str = if elem.0.starts_with( "r#" ) { &elem.0[2..] }
			                else { elem.0 }
			;


			write!( s, "{}=", key )?;

			val_to_string( &elem.1, s )?;

			for elem in iter
			{
				let key: &str = if elem.0.starts_with( "r#" ) { &elem.0[2..] }
				                else { elem.0 }
				;

				write!( s, ", {}=", key )?;
				val_to_string( &elem.1, s )?;
			}

			write!( s, " }}" )?;
		}

		_ => {}
	};

	Ok(())
}
