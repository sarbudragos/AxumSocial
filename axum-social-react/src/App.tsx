//import './App.css'
import { AppBar, Box, Button, Container,  Toolbar, Typography } from '@mui/material'
import { BrowserRouter, Link, Route, Routes } from 'react-router-dom'
import UserPage from './components/user/UserPage'
import AllUsersPage from './components/user/AllUsersPage'

function App() {
  return (
    <Container maxWidth={false} disableGutters component='main'>
      <BrowserRouter>

        <AppBar position='sticky'>
          <Toolbar>
            <Button component={Link} to={`/`}>
                <Typography variant="h5" sx={{color: 'black'}}>
                    Users
                </Typography>
              </Button>
          </Toolbar>
        </AppBar>

        
          <Routes>
            <Route path="/" element={<AllUsersPage/>} />
            <Route path="/user/:id" element={<UserPage/>} />
          </Routes>
        

      </BrowserRouter>
    </Container>
  )
}

export default App
