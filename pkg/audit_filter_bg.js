
const path = require('path').join(__dirname, 'audit_filter_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./audit_filter.js'] = require('./audit_filter.js');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
