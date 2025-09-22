import express from "express";

const app = express();
const PORT = 5000;

app.get("/", (req, res) => {
  res.json({ msg: "Hello from Node.js backend 🚀" });
});

app.listen(PORT, () => {
  console.log(`Node.js backend running on port ${PORT}`);
});
