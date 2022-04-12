<?php
namespace johnInjection\nested;

#@Inject(at = "PREPEND", target = "nested/view.php/$Ftest1", search = "echo \"test1\";")
function test1Mixin() {
    echo "this is test1 with a mixin";
}

#@Inject(at = "PREPEND", target = "nested/view.php/$Ftest1", search = r"test")
function test2Mixin() {
    echo "this is test2 with a mixin";
}