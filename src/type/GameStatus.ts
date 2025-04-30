export type LetterStatus = "Correct" | "Present" | "Absent";

export interface AnswerHistoryLetter {
  letter: string;
  status: LetterStatus;
}

export interface GameStatus {
  answer: string;
  histories: AnswerHistoryLetter[][];
  isClear: boolean;
  push?: (word: string) => GameStatus;
}
