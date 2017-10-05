#[macro_use]
extern crate helix;
extern crate docopt;
extern crate libc;

use docopt::ArgvMap;
use helix::sys::VALUE;
use helix::{ToError, ToRuby, ToRubyResult, sys};

extern "C" {
    pub fn rb_ary_entry(array: VALUE, offset: isize) -> VALUE;
}

#[derive(Clone,Debug)]
struct MyArgvMap(ArgvMap);

impl helix::FromRuby for MyArgvMap {
    type Checked = ();

    fn from_ruby(_: VALUE) -> helix::CheckResult<Self::Checked> {
        Err("Use Docopt.parse()".to_error())
    }

    fn from_checked(_: Self::Checked) -> MyArgvMap {
        unreachable!()
    }
}

ruby! {
    class Docopt {
        struct {
            options: MyArgvMap,
        }

        def initialize(helix, options: MyArgvMap) {
            Docopt { helix, options }
        }

        def parse(usage: String, argv: Vec<String>) -> Result<Docopt, String> {
            let result = docopt::Docopt::new(usage)
                .and_then(|d| d.help(false).argv(argv.into_iter()).parse());

            match result {
                Ok(args) => Ok(Docopt::new(MyArgvMap(args))),
                Err(error) => match error {
                    docopt::Error::WithProgramUsage(e, msg) => {
                        Err(format!("{}\n\n{}\n", e, msg))
                    },
                    e => {
                        Err(format!("{}", e))
                    }
                }
            }
        }

        #[ruby_name="[]"]
        def get(&self, key: String) -> ToRubyResult {
            match self.options.0.map.find(&key) {
                None => ().to_ruby(),
                Some(value) => match *value {
                    docopt::Value::Switch(value) => {
                        if value {
                            true.to_ruby()
                        } else {
                            false.to_ruby()
                        }
                    },
                    docopt::Value::Plain(Some(ref string)) => {
                        let ptr = string.as_ptr();
                        let len = string.len();
                        Ok(unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) })
                    },
                    docopt::Value::Plain(None) => {
                        ().to_ruby()
                    }
                    docopt::Value::List(ref vector) => {
                        let array = unsafe { sys::rb_ary_new_capa(vector.len() as isize) };
                        for item in vector {
                            let ptr = item.as_ptr();
                            let len = item.len();
                            let ruby_string = unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) };
                            unsafe { sys::rb_ary_push(array, ruby_string) };
                        }
                        Ok(array)
                    }
                    docopt::Value::Counted(uint) => uint.to_ruby()
                },
            }
        }
    }
}
