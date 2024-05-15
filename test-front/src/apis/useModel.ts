import useSWR, { KeyedMutator } from "swr";

type UseModelResult<T> =
  | {
      isLoading: true;
      error: null;
      data: null;
      mutate: KeyedMutator<T>;
    }
  | {
      isLoading: false;
      error: Error;
      data: null;
      mutate: KeyedMutator<T>;
    }
  | {
      isLoading: false;
      error: null;
      data: T;
      mutate: KeyedMutator<T>;
    };

const fetcher = (path: string) =>
  fetch(`http://localhost:8080/${path}`).then((res) => res.json());

const useModel = <T>(path: string): UseModelResult<T> => {
  const { data, error, mutate } = useSWR<T>(path, fetcher);

  if (error != null) {
    return {
      isLoading: false,
      error,
      data: null,
      mutate,
    };
  }

  if (data == null) {
    return {
      isLoading: true,
      error: null,
      data: null,
      mutate,
    };
  }

  return {
    isLoading: false,
    error: null,
    data,
    mutate,
  };
};

export default useModel;
