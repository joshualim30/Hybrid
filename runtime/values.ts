// values.ts - Values for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// export ValueType enum
export type ValueType = 
    | "null"
    | "number"
    | "boolean";

// export RuntimeValue interface
export interface RuntimeValue { type: ValueType; }

// export NullValue class
export interface NullValue extends RuntimeValue { type: "null"; value: null; }

// make null function
export function MAKE_NULL() {
    return { type: "null", value: null } as NullValue;
}

// export NumberValue class
export interface NumberValue extends RuntimeValue { type: "number"; value: number; }

// make number function
export function MAKE_NUMBER(number = 0) {
    return { type: "number", value: number } as NumberValue;
}

// export BooleanValue class
export interface BooleanValue extends RuntimeValue { type: "boolean"; value: boolean; }

// make boolean function
export function MAKE_BOOLEAN(boolean = false) {
    return { type: "boolean", value: boolean } as BooleanValue;
}

