export abstract class ValueObject<TInstance> {
    /**
     * Check for equality between value objects. It is equivalent to `!this.neq(rhs)`
     */
    abstract eq(rhs: TInstance): boolean;
    /**
     * Check for non equality between value objects. It is equivalent to `!this.eq(rhs)`.
     */
    neq(rhs: TInstance): boolean {
        return !this.eq(rhs);
    }
}
