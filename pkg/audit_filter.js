let wasm;
const { TextDecoder } = require(String.raw`util`);

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}
/**
* @returns {string}
*/
module.exports.version = function() {
    const retptr = 8;
    const ret = wasm.version(retptr);
    const memi32 = getInt32Memory();
    const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
    wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
    return v0;
};

let WASM_VECTOR_LEN = 0;

let cachegetNodeBufferMemory = null;
function getNodeBufferMemory() {
    if (cachegetNodeBufferMemory === null || cachegetNodeBufferMemory.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory = Buffer.from(wasm.memory.buffer);
    }
    return cachegetNodeBufferMemory;
}

function passStringToWasm(arg) {

    const size = Buffer.byteLength(arg);
    const ptr = wasm.__wbindgen_malloc(size);
    getNodeBufferMemory().write(arg, ptr, size);
    WASM_VECTOR_LEN = size;
    return ptr;
}
/**
* @param {string} audit_str
* @param {string} nsp_config_str
* @param {boolean} output_json
* @returns {number}
*/
module.exports.run_wasm = function(audit_str, nsp_config_str, output_json) {
    const ret = wasm.run_wasm(passStringToWasm(audit_str), WASM_VECTOR_LEN, passStringToWasm(nsp_config_str), WASM_VECTOR_LEN, output_json);
    return ret;
};

module.exports.__wbg_error_2c77153a1a48c1b3 = function(arg0, arg1) {
    console.error(getStringFromWasm(arg0, arg1));
};

module.exports.__wbg_log_958192cc4d464439 = function(arg0, arg1) {
    console.log(getStringFromWasm(arg0, arg1));
};
wasm = require('./audit_filter_bg');

