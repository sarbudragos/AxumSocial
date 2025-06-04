import { useEffect, useState } from "react";
import { User } from "../../model/User";
import { BASE_URL } from "../../consts";
import { Box, List, ListItem, Typography } from "@mui/material";
import UserCard from "./UserCard";

function AllUsersPage() {
  const [users, setUsers] = useState<User[]>([]);


  //let params = useParams()

  useEffect(() => {
      const fetchData = async () => {
        try {
            const response = await fetch(BASE_URL + `/users`)
            let user_list: User[] = (await response.json()).data
            console.log(user_list)
            setUsers(user_list)
        } catch (error) {
            console.log('Erro:', error)
        }
    }

    fetchData()
   }, []);


  return (
   //  <Typography variant="h1" component="h2">
   //    {user.username}
   // </Typography>

   <Box>
      <Typography variant="h2">
         Users:
      </Typography>

      <List>
         {users?.map(user => <ListItem key={user.id.toString()}><UserCard user={user}></UserCard></ListItem>)}
      </List>
   </Box>
  )
}

export default AllUsersPage