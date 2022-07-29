<?php
namespace johnInjection;
require_once($_SERVER["DOCUMENT_ROOT"] . "/test/injections/requires.php");

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

#@Inject(at = "SLICE", target = "index.php/$CController$Findex", from = 0, to = 3)
function JohnControllerSliceDefault(string &$var) {
    echo "<p>";
    echo "Sliced the default index output";
    echo $var;
    echo "</p>";
}

#@Inject(at = "SLICE", target = "index.php/$Findex", from = 0, to = 1)
function HelloJohnMixin() {
    echo "Hello, from John :)";
}

#@Inject(at = "HEAD", target = "index.php/$CController$Findex")
function JohnControllerModifyVarMixin(string &$var) {
    $var = "Variable changed in mixin!";
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

#@Inject(at = "TAIL", target = "index.php/$Findex", raw = true)
function GoodByeTailRawMixin() {
    echo "Raw injection!";
}
