export type MaybeClass = string | undefined | false | null

/** A builder class responsible for providing a friendly way to compound `css` classes */
export class ClassBuilder {
  private classes: string;

  constructor(from?: string) {
    this.classes = from ?? "";
  }

  /** Try to append `name` in class list if it's a valid class */
  add(name: MaybeClass): this {
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
