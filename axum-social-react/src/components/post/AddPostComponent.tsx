import { Button, Icon, Stack, TextField } from "@mui/material"
import type { User } from "../../model/User"
import { BASE_URL } from "../../consts";
import { useState } from "react";

function AddPostComponent({addPost} : {addPost: any}) {
    const [postInput, setPostInput] = useState('');

    const handlePostInputChange = (event: { target: { value: any; }; }) => {
        setPostInput(event.target.value);
    };

    // const onAdd = async () =>{
    //     const requestOptions = {
    //         method: 'POST',
    //         headers: { 'Content-Type': 'application/json' },
    //         body: JSON.stringify(
    //             {
    //                 content: postInput,
	//                 user_id: user.id,
	//                 creation_date: "2025-01-01T08:32:45.123"
    //             }
    //         )
    //     };
        

    //     const response = await fetch(BASE_URL + `/posts`, requestOptions)

    //     console.log(await response.json())
    // }

    const onAdd = async () => {
        addPost(postInput)
    }
  
  return (
    


    <Stack direction="row">
        <TextField value={postInput} onChange={handlePostInputChange}>
            Create post
        </TextField>
        <Button onClick={onAdd}>
            Add
        </Button>
    </Stack>

    
  )
}

export default AddPostComponent