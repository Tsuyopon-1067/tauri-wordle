import GridCell from "../atoms/GridCell";
import styles from "./WordRow.module.css";

interface WordRowProps {
  word: string;
  states?: ("correct" | "present" | "absent")[];
}

const WordRow = ({ word = "", states = [] }: WordRowProps) => {
  const letters = word.padEnd(5).slice(0, 5).split("");

  return (
    <div className={styles.wordRow}>
      {letters.map((letter, index) => (
        <GridCell key={index} letter={letter} state={states[index]} />
      ))}
    </div>
  );
};

export default WordRow;
