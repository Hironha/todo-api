export class ClassBuilder {
  private classes: string;

  constructor(from?: string) {
    this.classes = from ?? "";
  }

  add(name: string): this {
    if (name) {
      this.classes += this.classes ? ` ${name}` : name;
    }
    return this;
  }

  build(): string {
    return this.classes;
  }
}

export function classes(from?: string): ClassBuilder {
  return new ClassBuilder(from);
}
