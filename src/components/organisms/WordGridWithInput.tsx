import { useState } from "react";
import WordGrid from "./WordGrid";
import InputArea from "../molecules/InputArea";
import styles from "./WordGrid.module.css";

const WordGridWithInput = () => {
  const [words, setWords] = useState<string[]>([]);

  const handleSubmit = (word: string) => {
    setWords([...words, word]);
  };

  return (
    <div className={styles.wordGridContainer}>
      <WordGrid words={words} />
      <InputArea onSubmit={handleSubmit} />
    </div>
  );
};

export default WordGridWithInput;
