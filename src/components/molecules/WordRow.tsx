import { useEffect, useState } from 'react';
import { AnswerHistoryLetter } from '../../type/GameStatus';
import GridCell from '../atoms/GridCell';
import styles from './WordRow.module.css';

interface WordRowProps {
  histories: AnswerHistoryLetter[];
}

const WordRow = ({ histories }: WordRowProps) => {
  const DELAY_TIME_SECOND = 0.2;
  const [displayedRow, setDisplayedRow] = useState<AnswerHistoryLetter[]>([]);

  const animatedUpdate = (
    data: AnswerHistoryLetter[],
    setState: React.Dispatch<React.SetStateAction<AnswerHistoryLetter[]>>
  ) => {
    const emptyGridData = {
      letter: '',
      status: 'None',
    } as AnswerHistoryLetter;
    const emptyRowData = Array(5).fill(emptyGridData);
    setState(emptyRowData);
    data.forEach((item, index) => {
      setTimeout(
        () => {
          setState((prev) => {
            const prevSlice = prev.slice(0, index);
            const remainSlice = emptyRowData.slice(index + 1);
            return [...prevSlice, item, ...remainSlice];
          });
        },
        index * DELAY_TIME_SECOND * 1000
      );
    });
  };
  useEffect(() => {
    if (
      histories.length > 0 &&
      histories[0].letter !== displayedRow[0]?.letter
    ) {
      animatedUpdate(histories, setDisplayedRow);
    }
  }, [histories]);

  return (
    <div className={styles.wordRow}>
      {displayedRow.map((history, index) => (
        <GridCell key={index} letter={history.letter} status={history.status} />
      ))}
    </div>
  );
};

export default WordRow;
