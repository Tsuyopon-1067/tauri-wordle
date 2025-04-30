import { LetterStatus } from "../../type/GameStatus";
import styles from "./LetterBox.module.css";

interface LetterBoxProps {
  letter: string;
  status?: LetterStatus;
}

const LetterBox = ({ letter, status = "Absent" }: LetterBoxProps) => {
  const statusClass = styles[status.toLocaleLowerCase()];
  return <div className={`${styles.letterBox} ${statusClass}`}>{letter}</div>;
};

export default LetterBox;
