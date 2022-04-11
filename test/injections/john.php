<?php
namespace johnInjection;
require_once("./requires.php");

#@Inject(at = "HEAD", target = "index.php/$Findex")
function JohnSearchMixin() {
    echo "Looking for John...";
}

#@Inject(at = "HEAD", target = "index.php/$Findex", offset = 2)
function JohnMaybeMixin() {
    echo "Maybe he's here...";
}

#@Inject(at = "HEAD", target = "index.php/$CController$Findex")
function JohnControllerSearchMixin() {
    echo "Looking for John in the Controller...";
}

#@Inject(at = "HEAD", target = "nested/view.php/$Ftest")
function TestNestedMixin() {
    echo "The following will be printed: Test";
}

#@Inject(at = "SLICE", target = "index.php/$Findex", from = 0, to = 0)
function HelloJohnMixin() {
    echo "Hello, from John :)";
}

#@Inject(at = "HEAD", target = "index.php")
function HelloImportsMixin() {
    include "./imports.php";
}

#@Inject(at = "HEAD", target = "index.php")
function HelloRequiresMixin() {
    require_once "./requires.php";
}

#@Inject(at = "TAIL", target = "index.php/$Findex", offset = 1)
function ShowDirTailMixin() {
    echo get_dir();
}

#@Inject(at = "TAIL", target = "index.php/$Findex")
function GoodByeTailMixin() {
    echo "Goodbye!";
}
