import DeleteIcon from "@mui/icons-material/Delete";
import EditIcon from "@mui/icons-material/Edit";
import {
  IconButton,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from "@mui/material";

import { User } from "../types/User";

interface UsersTableProps {
  users: User[];
}

const UsersTable = ({ users }: UsersTableProps): JSX.Element => {
  return (
    <Table>
      <TableHead>
        <TableRow>
          <TableCell>ID</TableCell>
          <TableCell>名前</TableCell>
          <TableCell>メール</TableCell>
          <TableCell>編集</TableCell>
          <TableCell>削除</TableCell>
        </TableRow>
      </TableHead>
      <TableBody>
        {users.map((user) => (
          <TableRow key={user.id}>
            <TableCell>{user.id}</TableCell>
            <TableCell>{user.name}</TableCell>
            <TableCell>{user?.email}</TableCell>
            <TableCell>
              <IconButton>
                <EditIcon />
              </IconButton>
            </TableCell>
            <TableCell>
              <IconButton>
                <DeleteIcon />
              </IconButton>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
};

export default UsersTable;
