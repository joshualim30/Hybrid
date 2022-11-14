// ast.ts - AST Node Types for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// export NodeTypes
export type NodeType = 
    // Statements
    | "Program" 
    | "VariableDeclaration"
    // Expressions
    | "AssignmentExpression"
    | "NumberLiteral"
    | "Identifier" 
    | "BinaryExpression";

// export interface statements
export interface Statement { kind: NodeType; }

// export interface Expression
export interface Expression extends Statement { }

// export interface Program
export interface Program extends Statement { kind: "Program",  body: Statement[] }

// export interface VariableDeclaration 
export interface VariableDeclaration extends Statement { kind: "VariableDeclaration", constant: boolean, identifier: string, value?: Expression }

// export interface AssignmentExpression
export interface AssignmentExpression extends Expression { kind: "AssignmentExpression", assignee: Expression, value: Expression }

// export interface BinaryExpression
export interface BinaryExpression extends Expression { kind: "BinaryExpression",  left: Expression,  right: Expression,  operator: string }

// export interface Identifier
export interface Identifier extends Expression { kind: "Identifier",  symbol: string } // symbol is the name of the variable

// export interface NumberLiteral
export interface NumberLiteral extends Expression { kind: "NumberLiteral",  value: number }