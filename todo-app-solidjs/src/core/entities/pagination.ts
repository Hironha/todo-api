export type Pagination<T> = {
  perPage: number;
  page: number;
  count: number;
  data: T[];
};

export function getNextPage<T>(pagination: Pagination<T>): number | undefined {
  const currentTotal = pagination.perPage * pagination.page;
  if (currentTotal >= pagination.count) {
    return undefined;
  }

  return pagination.page + 1;
}

export function getTotalPages<T>(pagination: Pagination<T>): number {
  return Math.ceil(pagination.count / pagination.perPage);
}
