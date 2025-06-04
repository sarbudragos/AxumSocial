import { useEffect, useState } from "react"
import { Box, Button, Card, CardContent, List, ListItem, Stack, Typography} from "@mui/material";
import { BASE_URL } from "../../consts"
import { User } from "../../model/User"
import { useLocation, useParams } from "react-router-dom";
import { Post } from "../../model/Post";
import UserCard from "./UserCard";
import PostCard from "../post/PostCard";
import AddPostComponent from "../post/AddPostComponent";

function UserPage() {
  const [user, setUser] = useState<User>(new User);
  const [posts, setPosts] = useState<Post[]>([]);
  const [followingUsers, setFollowingUsers] = useState<User[]>([]);

  let params = useParams()

  useEffect(() => {

      const fetchData = async () => {
      try {
         const response = await fetch(BASE_URL + `/users/${params.id}`)
         let current_user: User = (await response.json()).data
         console.log(current_user)
         setUser(current_user)

         const response_posts = await fetch(BASE_URL + `/posts/user/${current_user.id}`)
         let current_posts: Post[] = (await response_posts.json()).data
         console.log(current_posts)
         setPosts(current_posts)

         const response_following = await fetch(BASE_URL + `/users/${current_user.id}/following`)
         let user_list: User[] = (await response_following.json())
         console.log(user_list)
         setFollowingUsers(user_list)
      } catch (error) {
        console.log('Erro:', error)
      }
    }

    fetchData()
   }, [params]);

   const addPost = async (content: String) => {
      const new_post = {
         user_id: user.id,
         content: content.toString(),
         creation_date: "2025-06-04T15:23:47.214399"
      }
      
      const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(new_post)
        };
        
      const response = await fetch(BASE_URL + `/posts`, requestOptions)

      const post = (await response.json()).data

      console.log(post)

      setPosts([...posts, post])
   }


  return (
   //  <Typography variant="h1" component="h2">
   //    {user.username}
   // </Typography>

   <Stack direction="row">
      <Box sx={{ flexGrow: 1 }}>
         <UserCard user={user}></UserCard>

         <Typography variant="h2">
            Posts:
         </Typography>

         <AddPostComponent addPost={addPost}></AddPostComponent>

         <List>
            {posts?.map(post => <ListItem key={post.id.toString()}><PostCard post={post}></PostCard></ListItem>)}
         </List>
      </Box>

      <Box>
      <Typography variant="h2">
         Following:
      </Typography>

      {followingUsers !== undefined && followingUsers.length > 0 && 
      <List>
         {followingUsers?.map(user => <ListItem key={user.id.toString()}><UserCard user={user}></UserCard></ListItem>)}
      </List>
      }
   </Box>
   </Stack>
  )
}

export default UserPage