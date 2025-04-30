import styles from './GridCell.module.css';
import LetterBox from './LetterBox';

interface GridCellProps {
  letter: string;
  state?: 'correct' | 'present' | 'absent';
}

const GridCell = ({ letter, state = 'absent' }: GridCellProps) => {
  return (
    <div className={styles.gridCell}>
      <LetterBox letter={letter} state={state} />
    </div>
  );
};

export default GridCell;
