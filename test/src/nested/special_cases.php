<?php

// Sometimes my PHP interpreter has trouble with parentisis in parentisis
// So here is a test case for that
function index(array $array = array()) {
    print_r($array);
}

index();