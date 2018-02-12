[![Build Status](https://travis-ci.org/ben-ole/codacy-xcov.svg?branch=master)](https://travis-ci.org/ben-ole/codacy-xcov)

# codacy-xcov
Generic parser for coverage reports to keep track on [codacy](https://www.codacy.com).

# usage

    USAGE:
        covrep [FLAGS] [OPTIONS] <PROJECT_TOKEN> <COMMIT> <INPUT>

    FLAGS:
        -h, --help       Prints help information
        -v               Sets the level of verbosity
        -V, --version    Prints version information

    OPTIONS:
        -o, --output <OUTPUT>        save compiled json to `output`.
            --prefix <PREFIX>        Prefix the path to the files in the output json with given slice.
        -t, --type <TYPE>            Set input file type. Available: `json`. defaults to `json`
        -l, --language <LANGUAGE>    change language parameter for codacy. defaults to `swift`
        -p, --parser <PARSER>        Set parser to use for coverage extraction. Available: `xcov`. defaults to `xcov`

    ARGS:
        <PROJECT_TOKEN>    Set the codacy project token
        <COMMIT>           Set the current GIT Commit UUID
        <INPUT>            Sets the input file to use

# install

    This project uses [trust](https://github.com/japaric/trust) CI/CD script. [Binary releases](https://github.com/ben-ole/codacy-xcov/releases) are available for many platforms.