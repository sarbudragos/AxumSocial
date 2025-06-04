import { Card, CardContent, Typography } from "@mui/material"
import type { Post } from "../../model/Post"

function PostCard({ post }: { post: Post }) {
  
  return (
    <Card>
        <CardContent sx={{bgcolor: 'primary.main'}}>
            <Typography variant="h5">
                {post.content}
            </Typography>
            <Typography sx={{ color: 'text.secondary'} } variant="h6">
                Posted on: {post.creation_date}
            </Typography>
        </CardContent>
    </Card>
  )
}

export default PostCard