var a = 100;
var b = 235.6;
var s = "hey";

assert_eq(a + b + 10, 345.6);
assert_eq(a - 10 + b, 325.6);
assert_eq(a - (10 + b), -145.6);

assert_eq(a + 10 + s, "110hey");
assert_eq(a + s + 10, "110hey");
assert_eq("20" + 15, "2015");

assert_eq(+"20" + 15, 35);
assert_eq(-"20" + 15, -5);

assert_eq(a * 5 * 10, 5000);
assert_eq(a / (5 * 10), 2);
assert_eq("3.5" * 5 * 10, 175);

assert_eq(+s, NaN);
assert_eq(s * 10, NaN);
assert_eq(s - a, NaN);
