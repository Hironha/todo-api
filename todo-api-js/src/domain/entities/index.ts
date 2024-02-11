import { type Id } from "@domain/value-objects/id";

export abstract class Entity<TProps> {
    protected constructor(protected props: TProps) {}

    abstract get id(): Id;

    unpack(): Readonly<TProps> {
        return Object.freeze({ ...this.props });
    }

    /**
     * Checks for equivalence between entities. Generally, entities are going to
     * be equivalent if they have the same `id`, i.e. they represent the same thing
     */
    eq(rhs: this): boolean {
        return this.id.value === rhs.id.value;
    }

    /**
     * Checks for non equivalence between entities. It's equivalent to `!this.eq(rhs)`
     */
    neq(rhs: this): boolean {
        return !this.eq(rhs);
    }
}
