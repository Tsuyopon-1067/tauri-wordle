export type LetterStatus = 'Correct' | 'Present' | 'Absent' | 'None';

export interface AnswerHistoryLetter {
  letter: string;
  status: LetterStatus;
}

export type AnswerHistory = AnswerHistoryLetter[][];
export interface GameStatus {
  answer: string;
  histories: AnswerHistory;
  isClear: boolean;
  push?: (word: string) => GameStatus;
}
