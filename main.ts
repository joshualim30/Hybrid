// main.ts - Main file for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// import necessary files & classes
import Parser from "./frontend/parser.ts";
import { evaluate } from "./runtime/interpreter.ts";

// run repl
repl();

// repl function
function repl() {

    const parser = new Parser();
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
        const result = evaluate(program);
        console.log(result);

    }

}