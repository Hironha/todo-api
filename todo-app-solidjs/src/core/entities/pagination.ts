export type Pagination<T> = {
  perPage: number;
  page: number;
  count: number;
  data: T[];
};

export class PaginationUtils {
  static getNextPage<T>(pagination: Pagination<T>): number | undefined {
    const currentTotal = pagination.perPage * pagination.page;
    if (currentTotal < pagination.count) {
      return pagination.page + 1;
    } else {
      return undefined;
    }
  }
}
