// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, {hello_str} from "./pkg/test3.js";

const runWasm = async () => {
    await init()
  // Instantiate our wasm module
//   const helloWorld = await init("./pkg/test3_bg.wasm");

  // Call the Add function export from wasm, save the result
//   const addResult = helloWorld.add(24, 24);

  // Set the result onto the body
   document.body.querySelector('.table_body').innerHTML = hello_str()
};
runWasm();

//1700-1900

// let tr = ''
// for(let i=0; i<10000; i++){
//     tr += '<tr>'
//     tr+=`<td>${i+1}</td>`
//     for(let j=0; j<30; j++){
//         tr+='<td>test</td>'
//     }
//     tr += '</tr>'
// }

// document.body.querySelector('.table_body').innerHTML = tr