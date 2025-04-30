import { AnswerHistoryLetter } from "../../type/GameStatus";
import GridCell from "../atoms/GridCell";
import styles from "./WordRow.module.css";

interface WordRowProps {
  histories: AnswerHistoryLetter[];
}

const WordRow = ({ histories }: WordRowProps) => {
  return (
    <div className={styles.wordRow}>
      {histories.map((history, index) => (
        <GridCell key={index} letter={history.letter} status={history.status} />
      ))}
    </div>
  );
};

export default WordRow;
