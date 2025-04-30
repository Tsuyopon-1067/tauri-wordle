import { useState } from 'react';
import WordGrid from './WordGrid';
import InputArea from '../molecules/InputArea';
import styles from './WordGrid.module.css';
import { invoke } from '@tauri-apps/api/core';
import { AnswerHistoryLetter, GameStatus } from '../../type/GameStatus';

const WordGridWithInput = () => {
  const [histories, setHistories] = useState<AnswerHistoryLetter[][]>([[]]);

  const handleSubmit = (word: string) => {
    invoke<GameStatus>('check_word', { word: word })
      .then((data) => setHistories(data.histories))
      .catch((error) => {
        console.error('Error checking word:', error);
      });
  };

  return (
    <div className={styles.wordGridContainer}>
      <WordGrid histories={histories} />
      <InputArea onSubmit={handleSubmit} />
    </div>
  );
};

export default WordGridWithInput;
