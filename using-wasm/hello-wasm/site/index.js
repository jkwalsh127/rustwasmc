// import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
//     js.greet("WebAssembly with npm");
//   })
//   .then((js) => {
//     js.add(10, 15);
//   });
  
  const math = require('../pkg/hello_wasm');

  console.log(math.add_numbers(10,20));