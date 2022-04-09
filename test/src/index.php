<?php
function index() {
    echo "hello, world";
}

index();

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