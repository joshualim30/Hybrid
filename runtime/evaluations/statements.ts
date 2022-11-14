import { Program, VariableDeclaration } from "../../frontend/ast.ts";
import Environment from "../environment.ts";
import { evaluate } from "../interpreter.ts";
import { MAKE_NULL, RuntimeValue } from "../values.ts";

// evaluate program function
export function evaluateProgram(program: Program, environment: Environment): RuntimeValue {
    let lastEvaluatedValue: RuntimeValue = MAKE_NULL();
    for (const statement of program.body) { 
        lastEvaluatedValue = evaluate(statement, environment);
    }
    return lastEvaluatedValue;
}

// evaluate variable declaration function
export function evaluateVariableDeclaration(declaration: VariableDeclaration, environment: Environment) {
    const value = declaration.value ? evaluate(declaration.value, environment) : MAKE_NULL();
    return environment.declareVariable(declaration.identifier, value, declaration.constant);
}