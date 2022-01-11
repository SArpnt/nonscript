lalrpop_mod!(pub syntax);

#[test]
fn testParser() {
	macro_rules! parseTest {
		($type:ident $code:literal $val:pat $(if $guard:expr)?,) => {
			assert!(matches!(paste!{syntax::[<$type Parser>]::new().parse($code)}, $val $(if $guard)?));
		};

		($type:ident $code:literal $val:pat $(if $guard:expr)?, $($typeN:ident $codeN:literal $valN:pat $(if $guardN:expr)?,)+) => {
			parseTest! { $type $code $val $(if $guard)?, }
			parseTest! { $($typeN $codeN $valN $(if $guardN)?,)+ }
		};
	}
	parseTest! {
		// garbage
		Term "" Err(_),
		Term "\\" Err(_),

		// booleans
		Boolean "true" Ok(true),
		Boolean "false" Ok(false),

		// numbers
		Number "1e3" Ok(x) if x == 1e3,
		Number "-632e-23" Ok(x) if x == -632e-23,
		Number "1E3" Err(_),
		Number "1e" Err(_),
		Number "e2" Err(_),
		Number "." Err(_),
	};
}

pub fn parse(_inString: String) -> Result<String, ()> {
	Err(())
}
