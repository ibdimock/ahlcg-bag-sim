
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @returns {ChaosBag}
*/
export function build_chaos_bag() {
    var ret = wasm.build_chaos_bag();
    return ChaosBag.__wrap(ret);
}

/**
*/
export function draw() {
    wasm.draw();
}

/**
*/
export const Token = Object.freeze({ PlusOne:0,"0":"PlusOne",Zero:1,"1":"Zero",MinusOne:2,"2":"MinusOne",MinusTwo:3,"3":"MinusTwo",MinusThree:4,"4":"MinusThree",MinusFour:5,"5":"MinusFour",MinusFive:6,"6":"MinusFive",MinusSix:7,"7":"MinusSix",MinusSeven:8,"8":"MinusSeven",MinusEight:9,"9":"MinusEight",Skull:10,"10":"Skull",Cultist:11,"11":"Cultist",ElderThing:12,"12":"ElderThing",Tablet:13,"13":"Tablet",ElderSign:14,"14":"ElderSign",AutoFail:15,"15":"AutoFail",Bless:16,"16":"Bless",Curse:17,"17":"Curse", });
/**
*/
export class ChaosBag {

    static __wrap(ptr) {
        const obj = Object.create(ChaosBag.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chaosbag_free(ptr);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('ahlcg_bag_simulator_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_alert_c6886314a9d0c61d = function(arg0, arg1) {
        alert(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

