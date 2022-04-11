# PHP Injector

The purpose of this project is to provide a tool that interprets, injects and
compiles injections of PHP code into other PHP files without modifying the 
original core files directly. Similar to a tool like VQMod for PHP OpenCart but
more robust and fast.

The aims for this project are:

- Support the feature set that VQMod has.
- Add native tooling to watch and compile files
- Enable/Disable injections
- PHP integration
- Add injection breakdown
- Add profiling

# How to use

There are a few things needed to get this tool going. First you have to have 3
different directories in your PHP project, one for injections, one for source
code (that you don't want to make changes to directly) and one for the cache or
compilation target. These can be anywhere inside your project but you have to
tell PHP Injector where they are by creating a config file:

```json
// injections.json
{
    "injections": "path/to/injections/",
    "src": "path/to/src/",
    "cache": "path/to/cache/",
    "use_document_root": true
}
```

Any php file inside the "src" and "injections" directory will be interpreted
and will affect the final compilation.

To use this config file you must add the `--config` flag to the command you
launch the tool with like so:

```bash
php-injector --config injections.json
```

The injector will compile your mixins and put them into the cache directory.
If you want PHP Injector to watch your injections for changes so you can
compile on save, you can run:

```bash
php-injector --config injections.json --watch
```

# Final Notes

I will add more documentation soon, once i figure out all that stuff in the
[To Do](TODO.md) file.

While you're here, don't forget to check if you can use this in your project,
comercial or not: [License](LICENSE).

Fair warning, there probably is profanity in some comments, PHP is not my
favourite language... Good Luck!