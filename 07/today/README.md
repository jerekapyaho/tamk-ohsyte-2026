# Today 07

TextFileProvider and CSVFileProvider are in place.
Configuration file is parsed from the configuration directory.
For testing, place the `compsci.csv` file in the configuration directory
and make a configuration file with the name `today.toml`
in the configuration directory:

    [[providers]]
    name = "compsci"
    kind = "csv"
    resource = "compsci.csv"

