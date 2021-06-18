/* tslint:disable */
/* eslint-disable */
/**
* @returns {ChaosBag}
*/
export function build_chaos_bag(): ChaosBag;
/**
*/
export function draw(): void;
/**
*/
export enum Token {
  PlusOne,
  Zero,
  MinusOne,
  MinusTwo,
  MinusThree,
  MinusFour,
  MinusFive,
  MinusSix,
  MinusSeven,
  MinusEight,
  Skull,
  Cultist,
  ElderThing,
  Tablet,
  ElderSign,
  AutoFail,
  Bless,
  Curse,
}
/**
*/
export class ChaosBag {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chaosbag_free: (a: number) => void;
  readonly build_chaos_bag: () => number;
  readonly draw: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
