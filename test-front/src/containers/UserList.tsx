import UsersTable from "../components/UsersTable";

const UserList = (): JSX.Element => {
  const users = [
    { id: 1, name: "Jon" },
    { id: 2, name: "Bob" },
    { id: 3, name: "Alice" },
  ];

  return <UsersTable users={users} />;
};

export default UserList;
