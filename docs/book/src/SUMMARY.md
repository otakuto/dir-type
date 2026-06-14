# Summary

- [Introduction](./introduction.md)
- [Installation](./installation.md)
- [Basics: check and the config file](./getting-started.md)

# Building blocks of the type

- [file — require a file to exist](./file.md)
- [dir — require a subdirectory to exist](./dir.md)
- [optional — allow an entry to be absent](./optional.md)
- [file + regex — file type with a pattern](./file-regex.md)
- [dir + regex — directory type with a pattern](./dir-regex.md)
- [min / max — count limits](./count.md)
- [one_of — require exactly one](./one-of.md)
- [any_of — require one or more](./any-of.md)
- [choice — require N to M](./choice.md)
- [rule / use — reusing types](./rule-use.md)
- [group — bundling records as a record](./group.md)

# Dynamic structure and mirroring

- [for — iterate over a collection](./for.md)
- [Passing values with `with` — string / number / bool / list](./rule-with-value.md)
- [Passing a rule with `with` — parameterize by record](./rule-with-rule.md)
- [match — branch on the type](./match.md)
- [fetch — collect without consuming](./fetch.md)
- [recursive — type structures of arbitrary depth](./recursive.md)
- [group-clone — reuse collected results elsewhere](./group-clone.md)
- [recursive-clone — mirror a tree](./recursive-clone.md)
- [recursive-flatten — flatten a tree](./recursive-flatten.md)
