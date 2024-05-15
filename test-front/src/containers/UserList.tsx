import { Button, TextField } from "@mui/material";
import axios from "axios";
import { useState } from "react";

import useUsers from "../apis/useUsers";
import UsersTable from "../components/UsersTable";

const UserList = (): JSX.Element => {
  const { isLoading, error, users, mutate } = useUsers();
  const [name, setName] = useState("");

  const postUser = async (name: string) => {
    if (name === "") return;
    await axios.post("http://localhost:8080/users", {
      name,
    });
    mutate();
  };

  if (isLoading) {
    return <>loading...</>;
  }

  if (error) {
    return <>error!!</>;
  }

  return (
    <>
      <TextField value={name} onChange={(e) => setName(e.target.value)} />
      <Button onClick={() => postUser(name)}>追加</Button>
      <UsersTable users={users} />
    </>
  );
};

export default UserList;
