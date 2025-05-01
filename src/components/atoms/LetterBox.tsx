import { useEffect, useRef, useState } from 'react';
import { LetterStatus } from '../../type/GameStatus';
import styles from './LetterBox.module.css';

interface LetterBoxProps {
  letter: string;
  status?: LetterStatus;
}

const LetterBox = ({ letter, status = 'None' }: LetterBoxProps) => {
  const [isFlipping, setIsFlipping] = useState(false);
  const prevTextRef = useRef('');

  useEffect(() => {
    if (prevTextRef.current === '' && letter !== '') {
      setIsFlipping(true);

      const animationTimer = setTimeout(() => {
        setIsFlipping(false);
      }, 500);

      return () => {
        clearTimeout(animationTimer);
      };
    }

    prevTextRef.current = letter;
  }, [letter]);

  const statusClass = styles[status.toLocaleLowerCase()];
  const isVisible = letter !== '' || status === 'None';
  return (
    <div
      className={`${styles.letterBox} ${isFlipping ? styles.flipActive : ''} ${statusClass}`}
      style={{
        visibility: isVisible ? 'visible' : 'hidden',
      }}
    >
      {letter}
    </div>
  );
};

export default LetterBox;
