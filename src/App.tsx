import "./App.css";
import WordGrid from "./components/organisms/WordGrid";

function App() {
  return (
    <div className="App">
      <WordGrid
        words={["HELLO", "WORLD", "REACT"]}
        states={[
          ["correct", "present", "absent", "absent", "absent"],
          ["absent", "absent", "correct", "absent", "present"],
          ["absent", "absent", "absent", "absent", "absent"],
        ]}
      />
    </div>
  );
}

export default App;
