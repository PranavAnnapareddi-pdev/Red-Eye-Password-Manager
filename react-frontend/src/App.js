import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";


function App() {
  const [account, setAccount] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");

  const handleAdd = async () => {
    try {
      const response = await invoke("add_password", { account, username, password });
      setMessage(response); // Display success message
    } catch (error) {
      setMessage("Failed to add password.");
      console.error("Error:", error);
    }
  };

  return (
    <div style={{ padding: "20px" }}>
      <h1>Red-Eye Password Manager</h1>

      <div>
        <label>Account:</label>
        <input
          type="text"
          value={account}
          onChange={(e) => setAccount(e.target.value)}
        />
      </div>

      <div>
        <label>Username:</label>
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
      </div>

      <div>
        <label>Password:</label>
        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>

      <button onClick={handleAdd}>Add Password</button>

      {message && <p>{message}</p>}
    </div>
  );
}

export default App;
