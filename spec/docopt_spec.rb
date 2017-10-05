require "spec_helper"

USAGE = <<USAGE
Naval Fate.

    Usage:
      naval_fate ship new <name>...
      naval_fate ship <name> move <x> <y> [--speed=<kn>] [--acc=<kns>]
      naval_fate ship shoot <x> <y>
      naval_fate mine (set|remove) <x> <y> [--moored | --drifting]
      naval_fate (-h | --help)
      naval_fate --version
      naval_fate (-o | --option)

    Options:
      -h --help     Show this screen.
      --version     Show version.
      --speed=<kn>  Speed in knots [default: 10].
      --acc=<kns>   Speed in knots per second.
      --moored      Moored (anchored) mine.
      --drifting    Drifting mine.
      -o --option   Test long and short option.
USAGE

describe "Docopt" do
  it "should parse and return bools" do
    argv = "naval_fate --help".split

    options = Docopt.parse(USAGE, argv)
    expect(options.get_bool("--help")).to be true
    expect(options["--help"]).to be true
  end
end
