import { useState } from 'react';
import WordGrid from './WordGrid';
import InputArea from '../molecules/InputArea';
import styles from './WordGrid.module.css';
import { invoke } from '@tauri-apps/api/core';
import { AnswerHistory } from '../../type/GameStatus';

const WordGridWithInput = () => {
  const [histories, setHistories] = useState<AnswerHistory>([]);

  const handleReset = () => {
    invoke<AnswerHistory>('reset', {})
      .then((histories) => setHistories(histories))
      .catch((error) => {
        console.error('Error resetting:', error);
      });
  };

  const handleSubmit = (word: string) => {
    invoke<AnswerHistory>('check_word', { word: word })
      .then((histories) => setHistories(histories))
      .catch((error) => {
        console.error('Error checking word:', error);
      });
  };

  return (
    <div className={styles.wordGridContainer}>
      <button onClick={handleReset} className={styles.resetButton}>
        リセット
      </button>
      <WordGrid histories={histories} />
      <InputArea onSubmit={handleSubmit} />
    </div>
  );
};

export default WordGridWithInput;
