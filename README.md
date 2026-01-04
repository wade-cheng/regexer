# regexer

A simple CLI tool to run a series of regex commands on a source document.

In `document.txt`

```
Whoops!  This document has a few common errors !


Well, as they say,  "shucks".
```

In `regexes.txt`:

```
// fix common errors
remove double space        "  " -> " "
remove excessive newlines  "(\n\s*){3,}" -> "\n\n"
fix spacing                " ([!?])" -> "$1"
the quote thing            ""([!?.])" -> "$1""
"foo" -> "bar"
// foo isn't found, which just means nothing happens for that command
// (notice that comments are optional)
```

After running:

```
regexer --file ./document.txt --replacements ./regexes.txt
```

`./document.txt.replaced` will contain

```
Whoops! This document has a few common errors!

Well, as they say, "shucks."
```
