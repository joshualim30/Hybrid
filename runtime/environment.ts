// environment.ts - Environment for Hybrid Language in TypeScript
// (c) 2022 by Joshua Lim - @joshualim30 (GitHub)
// License: MIT

import { RuntimeValue } from "./values.ts";

export default class Environment {

    // get parent environment & variables
    private parent?: Environment;
    private variables: Map<string, RuntimeValue>;
    private constants: Set<string>;

    // constructor for Environment class
    constructor(parentENV?: Environment) {
        this.parent = parentENV;
        this.variables = new Map();
        this.constants = new Set();
    }

    public declareVariable(name: string, value: RuntimeValue, constant: boolean): RuntimeValue {
        // check if variable already exists
        if (this.variables.has(name)) {
            throw `Variable ${name} has already been declared.`;
        }
        // if not, declare variable
        this.variables.set(name, value);
        // if constant, add to constant set
        if (constant) {
            this.constants.add(name);
        }
        // return value
        return value;
    }

    public assignVariable(name: string, value: RuntimeValue): RuntimeValue {
        // check if variable exists
        const environment = this.resolve(name);
        // cannot assign to constant
        if (environment.constants.has(name)) {
            throw `Cannot reasign constant "${name}" as it was already declared.`;
        }
        // set variable
        environment.variables.set(name, value);
        // return value
        return value;
    }

    public findVariable(name: string): RuntimeValue {
        // find location of variable
        const environment = this.resolve(name);
        // return variable
        return environment.variables.get(name) as RuntimeValue;
    }

    public resolve(name: string): Environment {
        // check if variable exists
        if (this.variables.has(name)) {
            return this;
        }
        // if not, check parent environment if it doesn't exists
        if (this.parent == undefined) {
            throw `Cannot resolve "${name}" because it does not exist.`;
        }
        // if it does, resolve parent environment
        return this.parent.resolve(name);
    }

}