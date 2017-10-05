#[macro_use]
extern crate helix;
extern crate docopt;

use docopt::ArgvMap;
use helix::sys::VALUE;
use helix::{FromRuby, ToError, sys};

extern "C" {
    pub fn rb_ary_entry(array: VALUE, offset: isize) -> VALUE;
}

#[derive(Clone,Debug)]
struct MyArgvMap(ArgvMap);

#[derive(Clone,Debug)]
struct MyVec<T>(Vec<T>);

impl helix::FromRuby for MyArgvMap {
    type Checked = ();

    fn from_ruby(_: VALUE) -> helix::CheckResult<Self::Checked> {
        Err("Use Docopt.parse()".to_error())
    }

    fn from_checked(_: Self::Checked) -> MyArgvMap {
        unreachable!()
    }
}

impl<T: FromRuby> helix::FromRuby for MyVec<T> {
    type Checked = Vec<T::Checked>;

    fn from_ruby(value: VALUE) -> helix::CheckResult<Self::Checked> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_ARRAY) } {
            let len = unsafe { sys::RARRAY_LEN(value) };
            let mut checked = Vec::with_capacity(len as usize);

            for i in 0..len {
                let entry = unsafe { rb_ary_entry(value, i) };

                match T::from_ruby(entry){
                    Ok(v) => checked.push(v),
                    Err(e) => return Err(e),
                }
            }

            Ok(checked)
        } else {
            Err("No implicit conversion into ArgvMap".to_error())
        }
    }

    fn from_checked(checked: Self::Checked) -> Self {
        MyVec(checked.into_iter().map(T::from_checked).collect())
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

        def parse(usage: String, argv: MyVec<String>) -> Result<Docopt, String> {
            let argv = argv.0;
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

        def get_bool(&self, key: String) -> bool {
            self.options.0.get_bool(&key)
        }

        def log(&self, string: String) {
            println!("{}", string);
        }

        def inspect(&self) {
            println!("{:?}", self)
        }
    }
}
