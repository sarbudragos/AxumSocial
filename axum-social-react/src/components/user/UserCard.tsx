import { Button, Card, CardContent, Typography } from "@mui/material"
import type { User } from "../../model/User"
import { Link } from "react-router-dom"

function UserCard({ user }: { user: User }) {
  
  return (
    <Card>
        <CardContent sx={{bgcolor: 'primary.main'}}>
            <Button component={Link} to={`/user/${user.id}`}>
                <Typography variant="h5" sx={{color: 'black'}}>
                    User: {user.username}
                </Typography>
            </Button>
            <Typography sx={{ color: 'text.secondary', mb: 1.5 }}>
                Email: {user.email}
            </Typography>
        </CardContent>
    </Card>
  )
}

export default UserCard