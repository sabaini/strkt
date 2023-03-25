# README

strkt is a simple utility to transform structured (tabular) data, by applying a global and per-record template to it.

Supported input formats are csv and json. Json data must be provided as an array of objects, e.g.

```
[
  {
    "name": "Donald",
    "age": "33",
    "city": "Entenhausen"
  },
  {
    "name": "Daisy",
    "age": "23",
    "city": "Entenhausen"
  }

]
```

Templates are expected to be in [Handlebars](https://handlebarsjs.com/) format.


Usage information:

```
USAGE:
    strkt [OPTIONS] <input_file> <record_template> <global_template>

ARGS:
    <input_file>         The input data file
    <record_template>    The record template file
    <global_template>    The global template file

OPTIONS:
    -h, --help             Print help information
    -o, --output <FILE>    The output file, or '-' for stdout
    -V, --version          Print version information

```
