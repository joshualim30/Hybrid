// values.ts - Values for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// export ValueType enum
export type ValueType = 
    | "null"
    | "number";

// export RuntimeValue interface
export interface RuntimeValue { type: ValueType; }

// export NullValue class
export interface NullValue extends RuntimeValue { type: "null"; value: "null"; }

// export NumberValue class
export interface NumberValue extends RuntimeValue { type: "number"; value: number; }