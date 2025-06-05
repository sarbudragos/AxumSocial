import { Card, CardContent, Typography } from "@mui/material"
import type { Post } from "../../model/Post"

function PostCard({ post }: { post: Post }) {

  const index_of_T = post.creation_date.indexOf('T');
  const truncated_creation_date = post.creation_date.substring(0, index_of_T);

  console.log(truncated_creation_date);
  
  return (
    <Card>
        <CardContent sx={{bgcolor: 'primary.main'}}>
            <Typography variant="h5">
                {post.content}
            </Typography>
            <Typography sx={{ color: 'text.secondary'} } variant="h6">
                Posted on: {truncated_creation_date}
            </Typography>
        </CardContent>
    </Card>
  )
}

export default PostCard