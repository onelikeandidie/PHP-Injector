<?php
function index() {
    echo "hello, world";
}

index();

class Controller {
    public function index(string $var = "Hello")
    {
       echo "<p>";
       echo $var;
       echo "</p>";
    }

    public static function getInstance() {
        throw new Exception("Error Processing Request", 1);
    }
}

$controller = new Controller();
$controller->index("Help!");