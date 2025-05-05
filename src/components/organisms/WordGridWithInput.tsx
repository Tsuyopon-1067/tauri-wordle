import { useState } from 'react';
import WordGrid from './WordGrid';
import InputArea from '../molecules/InputArea';
import styles from './WordGrid.module.css';
import { invoke } from '@tauri-apps/api/core';
import { AnswerHistory, AnswerHistoryResponse } from '../../type/GameStatus';

const WordGridWithInput = () => {
  const [histories, setHistories] = useState<AnswerHistory>([]);

  const handleReset = () => {
    invoke<AnswerHistoryResponse>('reset', {})
      .then((response) => {
        if (response.is_update) {
          setHistories(response.histories);
        }
      })
      .catch((error) => {
        console.error('Error resetting:', error);
      });
  };

  const handleSubmit = (word: string) => {
    invoke<AnswerHistoryResponse>('check_word', { word: word })
      .then((response) => {
        if (response.is_update) {
          setHistories(response.histories);
        }
      })
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
