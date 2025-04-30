import { AnswerHistoryLetter } from '../../type/GameStatus';
import WordRow from '../molecules/WordRow';
import styles from './WordGrid.module.css';

interface WordGridProps {
  histories: AnswerHistoryLetter[][];
  states?: ('correct' | 'present' | 'absent')[][];
}

const WordGrid = ({ histories = [] }: WordGridProps) => {
  const emptyGridData = {
    letter: '',
    status: 'Absent',
  } as AnswerHistoryLetter;
  const emptyRowData = Array(5).fill(emptyGridData);
  const paddedWords = [...histories].concat(
    Array(6 - histories.length).fill(emptyRowData)
  );

  return (
    <div className={styles.wordGrid}>
      {paddedWords.slice(0, 6).map((rowHistory, index) => (
        <WordRow key={index} histories={rowHistory} />
      ))}
    </div>
  );
};

export default WordGrid;
