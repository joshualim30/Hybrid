// lexer.js - Lexer for the Hybrid language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// MIT License

// export enum TokenType
export enum TokenType {
    // Literals
    Null,
    Number,
    Identifier,

    // Keywords
    Let,

    // Grouping
    OpenParen,
    CloseParen,

    // Operators
    Equals,
    BinaryOperator,
    EOF // end of file
}

// keywords array
const KEYWORDS: Record<string, TokenType> = {
    "let": TokenType.Let,
    "null": TokenType.Null,
};

// export interface Token
export interface Token { value: string, type: TokenType, }

// token function 
function token(value: string, type: TokenType): Token {
    return { value, type, };
}

// isAlpha function - checks if a string contains only alphabets
function isAlpha(str: string) {
    return str.toUpperCase() != str.toLowerCase();
}

// isInteger function - checks if a string contains only integers
function isInteger(str: string) {
    const c = str.charCodeAt(0);
    const bounds = ["0".charCodeAt(0), "9".charCodeAt(0)];
    return (c >= bounds[0] && c <= bounds[1]);
}

// isSkippable function - checks if a character is skippable
function isSkippable(str: string) {
    return str == " " || str == "\t" || str == "\n";
}

// export function tokenize
export function tokenize(sourceCode: string): Token[] {

    // initialize tokens array
    const tokens = new Array<Token>();
    const src = sourceCode.split("");

    // build each token until end of file - NOT VERY MEMORY EFFICIENT
    while (src.length > 0) {

        if (src[0] == "(") {

            tokens.push(token(src.shift()!, TokenType.OpenParen));

        } else if (src[0] == ")") {

            tokens.push(token(src.shift()!, TokenType.CloseParen));

        } else if (src[0] == "+" || src[0] == "-" || src[0] == "*" || src[0] == "/" || src[0] == "%") {

            tokens.push(token(src.shift()!, TokenType.BinaryOperator));

        } else if (src[0] == "=") {

            tokens.push(token(src.shift()!, TokenType.Equals));

        } else { // handle multi-character tokens

            if (isInteger(src[0])) { // buld number token

                let num = "";
                while (src.length > 0 && isInteger(src[0])) {
                    num += src.shift();
                }
                tokens.push(token(num, TokenType.Number));

            } else if (isAlpha(src[0])) {

                let identifier = ""; // foo || let
                while (src.length > 0 && isAlpha(src[0])) {
                    identifier += src.shift();
                }

                // check for reserved keywords
                const reserved = KEYWORDS[identifier];
                if (typeof reserved == "number") { // not a reserved keyword
                    tokens.push(token(identifier, reserved));
                } else { // reserved keyword
                    tokens.push(token(identifier, TokenType.Identifier));
                }

            } else if (isSkippable(src[0])) { // skip whitespace

                src.shift();

            } else { // invalid character

                console.error("Unrecognized character found in source: " + src[0]);
                Deno.exit(1);

            }

        }

    }

    tokens.push({ type: TokenType.EOF, value: "EndOfFile" });
    return tokens;

}