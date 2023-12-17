import { createStore } from "solid-js/store";

export type MutateFn<P, T> = (payload: P) => Promise<T>;
export type Mutation<P, T> = {
  loading: () => boolean;
  data: () => T | undefined;
  mutate: (payload: P) => Promise<T>;
};

type MutationStore<T> = { loading: boolean; data?: T };

export function useMutation<P, T>(mutateFn: MutateFn<P, T>): Mutation<P, T> {
  const [store, setStore] = createStore<MutationStore<T>>({ loading: false });

  const mutate = async (payload: P): Promise<T> => {
    setStore({ loading: true });
    const response = await mutateFn(payload);
    setStore({ loading: false, data: response });
    return response;
  };

  return {
    loading: () => store.loading,
    data: () => store.data,
    mutate,
  };
}
