macro_rules! error_code_enum {
	(
		$( #[$enum_attr:meta] )*
		pub enum $name:ident ($ty:ty) {
			$(
				$( #[$variant_attr:meta] )*
				$variant:ident
			),+ $(,)?
		}
	) => {
		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
		#[repr(u8)]
		$( #[$enum_attr] )*
		pub enum $name {
			OK = (ResultCode::OK as u8),
			$(
			$( #[$variant_attr] )*
			$variant
			),*,
		}

		impl ::core::convert::From<$ty> for $name {
			fn from(value: $ty) -> Self {
				match value {
					$( <$ty>::$variant => $name::$variant ),*,
				}
			}
		}

		impl ::core::convert::From<$name> for Result<(), $ty> {
			fn from(value: $name) -> Self {
				match value {
					$( $name::$variant => Err(<$ty>::$variant) ),*,
					$name::OK => Ok(())
				}
			}
		}

		impl From<Result<(), $ty>> for $name {
			fn from(res: Result<(), $ty>) -> Self {
				match res {
					Ok(_) => $name::OK,
					Err(err) => {
						err.into()
					}
				}
			}
		}

		impl From<$name> for ResultCode {
			fn from(res: $name) -> Self {
				return (unsafe { std::mem::transmute::<$name, ResultCode>(res) });
			}
		}

		/*impl From<Result<(), $ty>> for ResultCode {
			fn from(res: Result<(), $ty>) -> Self {
				$name::From(res).into()
			}
		}*/
	}
}
