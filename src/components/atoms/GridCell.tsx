import { LetterStatus } from "../../type/GameStatus";
import styles from "./GridCell.module.css";
import LetterBox from "./LetterBox";

interface GridCellProps {
  letter: string;
  status?: LetterStatus;
}

const GridCell = ({ letter, status = "Absent" }: GridCellProps) => {
  return (
    <div className={styles.gridCell}>
      <LetterBox letter={letter} status={status} />
    </div>
  );
};

export default GridCell;
