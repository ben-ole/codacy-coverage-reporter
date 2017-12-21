# codacy-xcov
Third party parser for swift coverage reports [xcov](https://github.com/nakiostudio/xcov) to keep track on [codacy](https://www.codacy.com).

# usage

    USAGE:
        codacy-xcov [FLAGS] [OPTIONS] <PROJECT_TOKEN> <COMMIT> <INPUT>

    FLAGS:
        -h, --help       Prints help information
        -v               Sets the level of verbosity
        -V, --version    Prints version information

    OPTIONS:
        -o, --output <OUTPUT>        save compiled json to `output`.
        -p, --prefix <PREFIX>        Prefix the path to the files in the output json with given slice.
        -l, --language <LANGUAGE>    change language parameter for codacy. defaults to `swift`

    ARGS:
        <PROJECT_TOKEN>    Set the codacy project token
        <COMMIT>           Set the current GIT Commit UUID
        <INPUT>            Sets the input file to use

# caveats

xcov json reports currently don't document coverage per line, but on method level which is in turn not supported in the codacy api. 