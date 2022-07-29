<?php
namespace johnInjection\nested;

#@Inject(at = "PREPEND", target = "nested/view.php/$Ftest1", search = "echo \"test1\";")
function test1Mixin() {
    echo "this is test1 with a mixin";
}

#@Inject(at = "APPEND", target = "nested/view.php/$Ftest2", search = "test2() {")
function test2Mixin() {
    echo "this is test2 with a mixin";
}

#@Inject(at = "APPEND", target = "nested/view.php/$Ftest3", panic = false, search = r"test2()\s*.*\n\}\n")
function test3Mixin() {
    function test3Function() {
        echo "test3 function";
    }
}

#@Inject(at = "REPLACE", target = "nested/view.php/$Ftest4", search = "echo \"test4\";")
function test4Mixin() {
    echo "this is test4 with a mixin";
}

#@Inject(at = "REPLACE", target = "nested/view.php/$Ftest7", search = "echo test7Hello(\"test7\");")
function test5Mixin() {
    test7Hello("test7str");
}

#@Inject(at = "REPLACE", target = "nested/view.php/$Ftest7", panic = false, search = "echo test7Hello(\"test7\");")
function test6Mixin() {
    test7Hello("test8str");
}