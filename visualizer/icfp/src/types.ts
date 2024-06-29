export type Indicator = "T" | "F" | "I" | "S" | "U" | "B" | "?" | "L" | "v";
export type Token = String;
export const INDICATORS = "TFISB?L".split("");
export const BINARY_OPERATORS = "+-*/%<>=|&.TD$".split("");

export type Tree = {
  nodes?: Tree[];
  value: any;
  type: Indicator;
};

