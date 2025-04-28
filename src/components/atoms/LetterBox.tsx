import styles from "./LetterBox.module.css";

interface LetterBoxProps {
  letter: string;
  state?: "correct" | "present" | "absent";
}

const LetterBox = ({ letter, state = "absent" }: LetterBoxProps) => {
  return <div className={`${styles.letterBox} ${styles[state]}`}>{letter}</div>;
};

export default LetterBox;
