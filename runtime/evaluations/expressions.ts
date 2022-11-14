import { AssignmentExpression, BinaryExpression, Identifier } from "../../frontend/ast.ts";
import Environment from "../environment.ts";
import { evaluate } from "../interpreter.ts";
import { MAKE_NULL, NumberValue, RuntimeValue } from "../values.ts";

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
export function evaluateBinaryExpresion(binaryExpression: BinaryExpression, environment: Environment): RuntimeValue {
    const left = evaluate(binaryExpression.left, environment);
    const right = evaluate(binaryExpression.right, environment);
    if (left.type == "number" && right.type == "number") {
        return evaluateNumericBinaryExpression(left as NumberValue, right as NumberValue, binaryExpression.operator);
    }
    // else return null
    return MAKE_NULL();
}

// evaluate identifier function
export function evaluateIdentifier(identifier: Identifier, environment: Environment): RuntimeValue {
    const value = environment.findVariable(identifier.symbol);
    return value;
}

export function evaluateAssignment(node: AssignmentExpression, environment: Environment): RuntimeValue {
    if (node.assignee.kind != "Identifier") {
        throw `Invalid LHS inside assignment expression: ${JSON.stringify(node.assignee)}`;
    }
    const varName = (node.assignee as Identifier).symbol;
    return environment.assignVariable(varName, evaluate(node.value, environment));
}