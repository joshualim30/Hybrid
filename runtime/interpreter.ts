// interpreter.ts - Interpreter for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// import necessary files & classes
import { NumberValue, RuntimeValue } from "./values.ts";
import { Program, BinaryExpression, NumberLiteral, Statement, Identifier, VariableDeclaration, AssignmentExpression } from "../frontend/ast.ts";
import Environment from "./environment.ts";
import { evaluateIdentifier, evaluateBinaryExpresion, evaluateAssignment } from "./evaluations/expressions.ts";
import { evaluateProgram, evaluateVariableDeclaration } from "./evaluations/statements.ts";

// export evaluate function
export function evaluate(astNode: Statement, environment: Environment): RuntimeValue {

    switch (astNode.kind) {
        case "NumberLiteral":
            return { type: "number", value: ((astNode as NumberLiteral).value) } as NumberValue;
        case "Identifier":
            return evaluateIdentifier(astNode as Identifier, environment);
        case "AssignmentExpression":
            return evaluateAssignment(astNode as AssignmentExpression, environment);
        case "BinaryExpression":
            return evaluateBinaryExpresion(astNode as BinaryExpression, environment);
        case "Program":
            return evaluateProgram(astNode as Program, environment);
        case "VariableDeclaration":
            return evaluateVariableDeclaration(astNode as VariableDeclaration, environment);
        default:
            console.log("AST Node not supported for interpretation.", astNode);
            Deno.exit(1);
    }

}