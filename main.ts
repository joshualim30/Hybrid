// main.ts - Main file for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// import necessary files & classes
import Parser from "./frontend/parser.ts";
import Environment from "./runtime/environment.ts";
import { evaluate } from "./runtime/interpreter.ts";
import { MAKE_BOOLEAN, MAKE_NULL, MAKE_NUMBER } from "./runtime/values.ts";

// run repl
repl();

// repl function
function repl() {

    const parser = new Parser();
    const environment = new Environment();
    // environment.declareVariable("x", MAKE_NUMBER(100), true);
    environment.declareVariable("true", MAKE_BOOLEAN(true), true);
    environment.declareVariable("false", MAKE_BOOLEAN(false), true);
    environment.declareVariable("null", MAKE_NULL(), true);
    console.log("\nHybrid Repl v1.00.0");
    while (true) {

        // show prompt
        const input = prompt("> ");

        // check if input is null or user typed "exit"
        if (!input || input.includes("exit")) {
            Deno.exit(1);
        }

        // produce AST from source code
        const program = parser.produceAST(input);
        // console.log(program);

        // evaluate program
        const result = evaluate(program, environment);
        console.log(result);

    }

}