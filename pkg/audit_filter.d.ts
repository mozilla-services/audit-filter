/* tslint:disable */
/**
* @returns {string} 
*/
export function version(): string;
/**
* @param {string} audit_str 
* @param {string} nsp_config_str 
* @param {boolean} output_json 
* @returns {number} 
*/
export function run_wasm(audit_str: string, nsp_config_str: string, output_json: boolean): number;
