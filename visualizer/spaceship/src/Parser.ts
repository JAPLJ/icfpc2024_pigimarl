import {
  Token,
  Tree,
} from 'src/types'

export class Parser {
  private tokens: Token[] = [];
  private index = 0;

  public parse(input: string): Tree {
    this.index = 0;
    this.tokens = this.tokenize(input);
    const tree = this.parseTree();
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

  private parseInt(body: Token): number {
    let result = 0;
    for (let i = 0; i < body.length; i++) {
      result = result * 94 + body[i].charCodeAt(0) - 33;
    }
    return result;
  }

  private parseTree(): Tree | undefined {
    const token = this.getNextToken();
    if (token === null) {
      return;
    }
    const indicator = token[0];
    const body = token.slice(1);
    if (indicator === "T" || indicator === "F") {
      return {
        value: indicator === "T" ? "true" : "false",
        type: indicator as "T" | "F",
      };
    }
    if (indicator === "I") {
      return {
        value: this.parseInt(body),
        type: "I",
      };
    }
    if (indicator === "S") {
      const strMap = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&\'()*+,-./:;<=>?@[\]^_`|~ \n';
      const chars = []
      for (let i = 0; i < body.length; i++) {
        chars.push(strMap[body[i].charCodeAt(0) - 33]);
      }
      return {
        value: `"${chars.join('')}"`,
        type: "S",
      };
    }
    if (indicator === "U") {
      return {
        value: `u ${body}`,
        nodes: [this.parseTree()!],
        type: "U",
      };
    }
    if (indicator === "B") {
      return {
        value: `b ${body}`,
        nodes: [this.parseTree()!, this.parseTree()!],
        type: "B",
      };
    }
    if (indicator === "?") {
      return {
        value: "if",
        nodes: [this.parseTree()!, this.parseTree()!, this.parseTree()!],
        type: "?",
      };
    }
    if (indicator === "L") {
      return {
        value: `λ${this.parseInt(body)}`,
        nodes: [this.parseTree()!],
        type: "L",
      };
    }
    if (indicator === "v") {
      return {
        value: `v${this.parseInt(body)}`,
        type: "v",
      };
    }
    console.error("Unknown indicator", indicator);
  }
}
