lalrpop_mod!(pub syntax);

#[test]
fn testParser() {
	macro_rules! parseTest {
		($r:ident $t:ident $code:expr;) => {paste!{
			assert!(syntax::[<$t Parser>]::new().parse($code).[<is_ $r>]());
		}};

		($r:ident $t:ident $e:expr; $($rs:ident $ts:ident $es:expr;)+) => {
			parseTest! { $r $t $e; }
			parseTest! { $($rs $ts $es;)+ }
		};
	}
	parseTest! {
		// garbage
		err Term "";
		err Term "\\";

		// booleans
		ok Boolean "true";
		ok Boolean "false";

		// numbers
		ok Number "1e3";
		ok Number "-632e-23";
		err Number "1E3";
		err Number "1e";
		err Number "e2";
		err Number ".";
	};
}

pub fn parse(inString: String) -> String {
	String::from("temp")
}
