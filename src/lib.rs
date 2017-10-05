#[macro_use]
extern crate helix;
extern crate docopt;

use docopt::ArgvMap;
use helix::sys::VALUE;
use helix::{ToError, ToRuby, ToRubyResult};

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
                    docopt::Value::Counted(uint) => uint.to_ruby(),
                    docopt::Value::Plain(None) => ().to_ruby(),
                    ref plain @ docopt::Value::Plain(Some(_)) => plain.as_str().to_ruby(),
                    ref switch @ docopt::Value::Switch(_) => switch.as_bool().to_ruby(),
                    ref list @ docopt::Value::List(_) => list.as_vec().to_ruby()
                },
            }
        }
    }
}
