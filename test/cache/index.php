<?php
#mixin johnInjection/JohnControllerSearchMixin from ./test/./injections/john.php
require_once $_SERVER['DOCUMENT_ROOT'] . "/./test/./injections/john.php";
use function johnInjection/JohnControllerSearchMixin;

#mixin johnInjection/JohnMaybeMixin from ./test/./injections/john.php
require_once $_SERVER['DOCUMENT_ROOT'] . "/./test/./injections/john.php";
use function johnInjection/JohnMaybeMixin;

#mixin johnInjection/JohnSearchMixin from ./test/./injections/john.php
require_once $_SERVER['DOCUMENT_ROOT'] . "/./test/./injections/john.php";
use function johnInjection/JohnSearchMixin;

function index() {
JohnSearchMixin(); #mixin call JohnSearchMixin from ./test/./injections/john.php
    echo "hello, world";
JohnMaybeMixin(); #mixin call JohnMaybeMixin from ./test/./injections/john.php
}

index();

JohnControllerSearchMixin(); #mixin call JohnControllerSearchMixin from ./test/./injections/john.php
class Controller {
    public function index(Type $var = null)
    {
       echo "<html>";
       echo "</html>";
    }

    public static function getInstance() {
        throw new Exception("Error Processing Request", 1);
    }
}