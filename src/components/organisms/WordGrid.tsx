import WordRow from '../molecules/WordRow';
import styles from './WordGrid.module.css';

interface WordGridProps {
  words: string[];
  states?: ('correct' | 'present' | 'absent')[][];
}

const WordGrid = ({ words = [], states = [] }: WordGridProps) => {
  const paddedWords = [...words].concat(Array(6 - words.length).fill(''));

  return (
    <div className={styles.wordGrid}>
      {paddedWords.slice(0, 6).map((word, index) => (
        <WordRow key={index} word={word} states={states[index]} />
      ))}
    </div>
  );
};

export default WordGrid;
