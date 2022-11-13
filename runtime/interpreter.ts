// interpreter.ts - Interpreter for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// import necessary files & classes
import { NullValue, NumberValue, ValueType, RuntimeValue } from "./values.ts";
import { Program, BinaryExpression, NodeType, NumberLiteral, Statement } from "../frontend/ast.ts";

// evaluate numeric binary expression
function evaluateNumericBinaryExpression(left: NumberValue, right: NumberValue, operator: string): NumberValue {

    let result = 0;
    if (operator == "+") {
        result = left.value + right.value;
    } else if (operator == "-") {
        result = left.value - right.value;
    } else if (operator == "*") {
        result = left.value * right.value;
    } else if (operator == "/") {
        // TODO: handle division by zero
        result = left.value / right.value;
    } else if (operator == "%") {
        result = left.value % right.value;
    } else {
        console.log("Invalid operator:", operator);
        Deno.exit(1);
    }

    return { type: "number", value: result } as NumberValue;

}

// evaluate binary expression
function evaluateBinaryExpresion(binaryExpression: BinaryExpression): RuntimeValue {
    const left = evaluate(binaryExpression.left);
    const right = evaluate(binaryExpression.right);
    if (left.type == "number" && right.type == "number") {
        return evaluateNumericBinaryExpression(left as NumberValue, right as NumberValue, binaryExpression.operator);
    }
    // else return null
    return { type: "null", value: "null" } as NullValue;
}

// evaluate program function
function evaluateProgram(program: Program): RuntimeValue {
    let lastEvaluatedValue: RuntimeValue = { type: "null", value: "null" } as NullValue;
    for (const statement of program.body) { 
        lastEvaluatedValue = evaluate(statement);
    }
    return lastEvaluatedValue;
}

// export evaluate function
export function evaluate(astNode: Statement): RuntimeValue {

    switch (astNode.kind) {
        case "NumberLiteral":
            return { type: "number", value: ((astNode as NumberLiteral).value) } as NumberValue;
        case "NullLiteral":
            return { type: "null", value: "null" } as NullValue;
        case "BinaryExpression":
            return evaluateBinaryExpresion(astNode as BinaryExpression);
        case "Program":
            return evaluateProgram(astNode as Program);
        default:
            console.log("AST Node not supported for interpretation.", astNode);
            Deno.exit(1);
    }

}