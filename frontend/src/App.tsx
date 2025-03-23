import { useState } from "react";

function App() {
  const [text, setText] = useState("");
  const [response, setResponse] = useState("");

  const submit = async () => {
    try {
      const res = await fetch("http://localhost:3000/submit", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ text })
      });

      if (!res.ok) throw new Error("Failed to submit");

      const data = await res.json();
      setResponse(data.status);
      setText("");
    } catch (err) {
      console.error(err);
      setResponse("Error submitting text");
    }
  };

  return (
    <div style={{ padding: 20 }}>
      <h1>Tracknotes</h1>
      <textarea
        rows={4}
        cols={40}
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Enter text to submit"
      />
      <br />
      <button onClick={submit}>Submit</button>
      <p>Server says: {response}</p>
    </div>
  );
}

export default App;

