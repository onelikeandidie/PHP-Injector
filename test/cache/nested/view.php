<?php
#mixin johnInjection/TestNestedMixin from ./test/./injections/john.php
require_once $_SERVER['DOCUMENT_ROOT'] . "/./test/./injections/john.php";
use function johnInjection/TestNestedMixin;

function test() {
TestNestedMixin(); #mixin call TestNestedMixin from ./test/./injections/john.php
    echo "Test";
}