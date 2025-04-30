import { useState } from 'react';
import styles from './InputArea.module.css';

interface InputAreaProps {
  onSubmit: (word: string) => void;
}

const InputArea = ({ onSubmit }: InputAreaProps) => {
  const [input, setInput] = useState('');
  const [error, setError] = useState('');

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value.toUpperCase();
    if (value.length > 5) return;
    if (!/^[A-Z]*$/.test(value)) {
      setError('アルファベットのみ入力可能です');
      return;
    }
    setInput(value);
    setError('');
  };

  const handleSubmit = () => {
    if (input.length === 5) {
      onSubmit(input);
      setInput('');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSubmit();
    }
  };

  return (
    <div className={styles.container}>
      <input
        type='text'
        value={input}
        onChange={handleInputChange}
        onKeyDown={handleKeyDown}
        className={styles.input}
        maxLength={5}
      />
      {error && <p className={styles.error}>{error}</p>}
      <button
        onClick={handleSubmit}
        disabled={input.length !== 5}
        className={styles.button}
      >
        決定
      </button>
    </div>
  );
};

export default InputArea;
