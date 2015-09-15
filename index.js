var from_scratch = require("bindings")("from_scratch.node");

console.log(from_scratch.make_a_pi());
console.log(from_scratch.make_an_array());
console.log(from_scratch.make_an_array2());

var ITERATIONS = 100000;

function bench1() {
  var start = process.hrtime();
  for (var i = 0; i < ITERATIONS; i++) {
    from_scratch.make_an_array();
  }
  var diff = process.hrtime(start);
  return diff[0] * 1e9 + diff[1];
}

function bench2() {
  var start = process.hrtime();
  for (var i = 0; i < ITERATIONS; i++) {
    from_scratch.make_an_array2();
  }
  var diff = process.hrtime(start);
  return diff[0] * 1e9 + diff[1];
}

var raii = bench1();
console.log("RAII:  " + (raii / 1e6) + "ns / call");
var thunk = bench2();
console.log("thunk: " + (thunk / 1e6) + "ns / call");
console.log("diff:  " + ((thunk - raii) / 1e6) + "ns / call");
