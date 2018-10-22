//! Reusable types, function and macros for implementing serializers.

/// This macro provides a straightforward implementation of the default functions
/// of a serializer module.
macro_rules! def_default_api {
    ($writer: ident, $stringifier: ident) => {
        /// Shortcut for `Config::default().writer(write)`
        #[inline]
        pub fn writer<W: ::std::io::Write>(write: W) -> $writer<W> {
            Config::default().writer(write)
        }

        /// Shortcut for `Config::default().stringifier()`
        #[inline]
        pub fn stringifier() -> $stringifier {
            Config::default().stringifier()
        }
    };
    () => {
        def_default_api!(Writer, Stringifier);
    };
}


/// This macro provides a straightforward implementation of the `Stringifier` type,
/// based on the `Writer` type.
macro_rules! def_stringifier {
    ($writer: ident, $stringifier: ident) => {
        pub struct $stringifier {
            writer: $writer<Vec<u8>>,
        }

        impl StringSerializer for $stringifier {
            type Config = Config;

            fn new(config: Config) -> Stringifier {
                $stringifier{ writer: $writer::new(Vec::new(), config) }
            }
        }

        impl TripleSink for $stringifier {
            type Error = ();
            type Outcome = String;

            fn feed<T: Triple>(&mut self, t: &T) -> Result<(), ()> {
                self.writer.feed(t).map_err(|_| ())
            }

            fn finish(&mut self) -> Result<String, ()> {
                let mut v = Vec::new();
                swap(&mut self.writer.write, &mut v);
                Ok(unsafe { String::from_utf8_unchecked(v) })
            }
        }
    };
    () => {
        def_stringifier!(Writer, Stringifier);
    };
}