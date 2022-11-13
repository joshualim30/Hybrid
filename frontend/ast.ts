// ast.ts - AST Node Types for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// export NodeTypes
export type NodeType = 
    | "Program" 
    | "NumberLiteral" 
    | "NullLiteral"
    | "Identifier" 
    | "BinaryExpression";

// export interface statements
export interface Statement { kind: NodeType; }

// export interface Program
export interface Program extends Statement { kind: "Program",  body: Statement[], }

// export interface Expression
export interface Expression extends Statement { }

// export interface BinaryExpression
export interface BinaryExpression extends Expression { kind: "BinaryExpression",  left: Expression,  right: Expression,  operator: string }

// export interface Identifier
export interface Identifier extends Expression { kind: "Identifier",  symbol: string } // symbol is the name of the variable

// export interface NumberLiteral
export interface NumberLiteral extends Expression { kind: "NumberLiteral",  value: number }

// export interface NullLiteral
export interface NullLiteral extends Expression { kind: "NullLiteral",  value: "null" }