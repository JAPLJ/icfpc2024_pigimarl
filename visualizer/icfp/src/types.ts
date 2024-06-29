export type Operand = String | Number;
export type Operator = String;
export type Indicator = "T" | "F" | "I" | "S" | "B" | "?" | "L";
export type Token = String;
export const INDICATORS = "TFISB?L".split("");
export const BINARY_OPERATORS = "+-*/%<>=|&.TD$".split("");

export type Tree = {
  nodes?: Tree[];
  value: any;
};

