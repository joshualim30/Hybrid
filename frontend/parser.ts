// parser.ts - Parser for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

// import ast.ts
import { Statement, Program, Expression, BinaryExpression, NumberLiteral, Identifier, NullLiteral } from "./ast.ts";

// import lexer.ts
import { tokenize, Token, TokenType } from "./lexer.ts";

// export defualt class Parser
export default class Parser {

    // tokens array
    private tokens: Token[]= [];

    // private not end of file function
    private notEOF(): boolean {
        return this.tokens[0].type != TokenType.EOF;
    }

    // private at function - get current token at index
    private at() {
        return this.tokens[0] as Token;
    }

    // private next function - get next token
    private next() { // also known as "eat"
        const prev = this.tokens.shift() as Token;
        return prev;
    }

    // private expect function - expect a token, throw error if not found
    private expect(type: TokenType, error: string) {
        const prev = this.tokens.shift() as Token;
        if (!prev || prev.type != type) {
            console.error("Parser Error:\n", error, prev, " - Expecting: ", type);
            Deno.exit(1);
        }
        return prev;
    }

    // produce AST function - produces an abstract syntax tree from the tokens
    public produceAST(sourceCode: string): Program {

        // tokenize source code
        this.tokens = tokenize(sourceCode);
        const program: Program = { kind: "Program", body: [], };

        // parse until end of file
        while (this.notEOF()) { 

            program.body.push(this.parseStatement());

        }

        // parse tokens
        return program;

    }

    // private parse statement function
    private parseStatement(): Statement {
        
        // skip to parseExpression() for now
        return this.parseExpression();

    }

    // private parse expression function
    private parseExpression(): Expression {

        return this.parseAdditiveExpression(); // skip to parseAdditiveExpression() for now to follow the order of precedence

    }

    // private parse additive expression function
    private parseAdditiveExpression(): Expression {

        // parse multiplicative expression - left side of the expression
        let left = this.parseMultiplicativeExpression();

        // 
        while (this.at().value == "+" || this.at().value == "-") {

            // get operator
            const operator = this.next().value;

            // get right side of the expression
            const right = this.parseMultiplicativeExpression();

            // declare left as a binary expression
            left = { kind: "BinaryExpression", left, right, operator } as BinaryExpression;

        }

        // return left side of the expression
        return left;

    }

    // private parse multiplicative expression function
    private parseMultiplicativeExpression(): Expression {

        // parse primary expression - left side of the expression
        let left = this.parsePrimaryExpression();

        // 
        while (this.at().value == "*" || this.at().value == "/" || this.at().value == "%") {

            // get operator
            const operator = this.next().value;

            // get right side of the expression
            const right = this.parsePrimaryExpression();

            // declare left as a binary expression
            left = { kind: "BinaryExpression", left, right, operator } as BinaryExpression;

        }

        // return left side of the expression
        return left;

    }

    // Orders of Precedence:
    // - Assignment Expression
    // - Member Expression
    // - Function Call
    // - Logical Expression
    // - Comparison Expression
    // - Additive Expression
    // - Multiplicative Expression
    // - Unary Expression
    // - Primary Expression

    // primary parse expression function
    private parsePrimaryExpression(): Expression {

        const token = this.at().type;

        switch(token) {
            case TokenType.Null:
                this.next(); // advance to next token
                return { kind: "NullLiteral", value: "null" } as NullLiteral;
            case TokenType.Identifier:
                return { kind: "Identifier", symbol: this.next().value } as Identifier;
            case TokenType.Number:
                return { kind: "NumberLiteral", value: parseFloat(this.next().value) } as NumberLiteral;
            case TokenType.OpenParen: {
                this.next(); // eat "("
                const value = this.parseExpression();
                this.expect(TokenType.CloseParen, "Expected closing parenthesis not found after expression");
                return value;
            }
            default:
                console.error("Unexpected token found during parsing: " + this.at().value);
                Deno.exit(1);
        }

    }

}