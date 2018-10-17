
const path = require('path').join(__dirname, 'audit_filter_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./audit_filter'] = require('./audit_filter');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
