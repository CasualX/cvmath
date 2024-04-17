/*!
Macros to help with codegen of dimensional variants.
*/

#![macro_use]

/// Expands a list of expressions by inserting an infix operator between them.
macro_rules! infix {
	($op:tt $e:expr) => ($e);
	($op:tt $e:expr, $($tail:expr),+) => ($e $op infix!($op $($tail),+));
}

/// Expands a list of statements by inserting a statement between them.
macro_rules! instmt {
	($tween:stmt; $s:stmt;) => ($s);
	($tween:stmt; $s:stmt; $($tail:stmt;)+) => ($s $tween instmt!($tween; $($tail;)+));
}

/// Folds a list of expressions.
macro_rules! fold {
	($f:expr, $e:expr) => ($e);
	($f:expr, $acc:expr, $e:expr $(,$tail:expr)*) => (fold!($f, $f($acc, $e) $(,$tail)*));
}

/// Defines a specialized type with a given number of components.
macro_rules! specialized_type {
	($base:ident, $id:ident, $ty:ty, $($c:ident),+) => {
		#[doc = stringify!($base)]
		#[doc = "with"]
		#[doc = stringify!($ty)]
		#[doc = "components."]
		pub type $id = $base<$ty>;

		#[doc = stringify!($id)]
		#[doc = "constructor."]
		#[allow(non_snake_case)]
		#[inline]
		pub const fn $id($($c: $ty),+) -> $id { $base { $($c),+ } }
	};
}
