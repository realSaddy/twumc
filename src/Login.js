import React, { useState } from "react";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  function handleSubmit(e) {
    console.log(username);
    console.log(password);

    e.preventDefault();
  }

  return (
    <div>
      <Stack
        component="form"
        spacing={3}
        noValidate
        onSubmit={handleSubmit}
        sx={{
          width: "35ch",
          marginLeft: "auto",
          marginRight: "auto",
          marginTop: "25%",
        }}
      >
        <TextField
          id="username"
          label="Username"
          variant="standard"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <TextField
          id="password"
          label="Password"
          variant="standard"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <Button type="submit" variant="contained">
          Login
        </Button>
      </Stack>
    </div>
  );
}
