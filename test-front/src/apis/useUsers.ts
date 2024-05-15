import { KeyedMutator } from "swr";

import useModel from "../apis/useModel";
import { User } from "../types/User";

interface UsersResponse {
  total: number;
  data: User[];
}

type UseUsersResult =
  | {
      isLoading: true;
      error: null;
      users: null;
      mutate: KeyedMutator<UsersResponse>;
    }
  | {
      isLoading: false;
      error: Error;
      users: null;
      mutate: KeyedMutator<UsersResponse>;
    }
  | {
      isLoading: false;
      error: null;
      users: User[];
      mutate: KeyedMutator<UsersResponse>;
    };

const useUsers = (): UseUsersResult => {
  const { error, data, mutate } = useModel<UsersResponse>("users");

  if (error != null) {
    return {
      isLoading: false,
      error,
      users: null,
      mutate,
    };
  }

  if (data == null) {
    return {
      isLoading: true,
      error: null,
      users: null,
      mutate,
    };
  }

  return {
    isLoading: false,
    error: null,
    users: data.data,
    mutate,
  };
};

export default useUsers;
