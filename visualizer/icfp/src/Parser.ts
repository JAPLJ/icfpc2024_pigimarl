import {
  Token,
  BINARY_OPERATORS,
  INDICATORS,
  Tree,
} from 'src/types'

export class Parser {
  private tokens: Token[] = [];
  private index = 0;

  public parse(input: string): Tree {
    this.tokens = this.tokenize(input);
    const tree = this.parseTree();
    console.log(tree);
    return tree!;
  }

  private tokenize(input: string): Token[] {
    return input.split('\n')
    .filter((line) => line.length > 0)
    .map((line) => line.split(' '))
    .reduce((acc, line) => acc.concat(line), []);
  }

  private getNextToken(): Token | null{
    if (this.index >= this.tokens.length) {
      return null;
    }
    return this.tokens[this.index++];
  }

  private parseTree(): Tree | undefined {
    const token = this.getNextToken();
    if (token === null) {
      return;
    }
    const indicator = token[0];
    const body = token.slice(1);
    console.log(indicator, "body", body);
    if (indicator === "T" || indicator === "F") {
      return {
        value: indicator === "T",
      };
    }
    if (indicator === "I") {
      let result = 0;
      for (let i = 0; i < body.length; i++) {
        result = result * 94 + body[i].charCodeAt(0) - 33;
      }
      return {
        value: `integer ${result}`,
      };
    }
    if (indicator === "S") {
      // const strMap = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&\'()*+,-./:;<=>?@[\]^_`|~ \n';
      return {
        value: `string ${body}`,
      };
    }
    if (indicator === "U") {
      return {
        value: `unary ${body}`,
        nodes: [this.parseTree()!],
      };
    }
    if (indicator === "B") {
      return {
        value: `binary ${body}`,
        nodes: [this.parseTree()!, this.parseTree()!],
      };
    }
    if (indicator === "?") {
      return {
        value: "if",
        nodes: [this.parseTree()!, this.parseTree()!, this.parseTree()!],
      };
    }
    if (indicator === "L") {
      return {
        value: `lambda ${body}`,
        nodes: [this.parseTree()!],
      };
    }
    if (indicator === "v") {
      return {
        value: `variable ${body}`,
      };
    }
  }

}
