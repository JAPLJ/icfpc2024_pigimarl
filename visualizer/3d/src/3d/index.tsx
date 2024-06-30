import { Visualizer } from 'src/3d/Visualizer';
import { useEffect, useState } from 'react';
import { CellType } from 'src/types';

type ThreeDEmulatorProps = {
  initialField: CellType[][];
};

export const ThreeDEmulator = ({ initialField }: ThreeDEmulatorProps) => {
  const [tick, setTick] = useState(0);
  const [totalTick, setTotalTick] = useState(0);
  const [valA, setValA] = useState(0);
  const [valB, setValB] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [answer, setAnswer] = useState<number | null>(null);
  const [fieldHist, setFieldHist] = useState<CellType[][][]>([]);

  useEffect(() => {
    initialize();
  }, [initialField, valA, valB]);

  const initialize = () => {
    if (initialField.length === 0) {
      return;
    }
    setError(null);
    setAnswer(null);
    setTick(0);
    setTotalTick(0);
    const h = initialField.length;
    const w = initialField[0].length;
    const field = Array.from(Array(h + 2), () => Array(w + 2).fill("."));
    initialField.map((row, i) => row.map((cell, j) => {
      if (cell === "A") {
        field[i + 1][j + 1] = valA;
      } else if (cell === "B") {
        field[i + 1][j + 1] = valB;
      } else if (!Number.isNaN(parseInt(cell))) {
        field[i + 1][j + 1] = parseInt(cell);
      } else {
        field[i + 1][j + 1] = cell;
      }
    }));
    setFieldHist([field]);
  }

  const onChangeValA = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValA(parseInt(event.target.value));
  }

  const onChangeValB = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValB(parseInt(event.target.value));
  }

  const isNumber = (value: CellType): value is Number => {
    return typeof value === "number";
  }

  const nextTick = () => {
    const field = fieldHist[tick];
    const nextField = field.map(row => row.map(cell => cell));
    const writeField = field.map(row => row.map(() => null));
    let stop = true;
    const write = (i: number, j: number, value: number) => {
      if (writeField[i][j] !== null) {
        setError(`Multiple writes at (${i}, ${j})`);
        return;
      }
      stop = false;
      writeField[i][j] = value;
    }
    let warpTick = null;
    let warpField = fieldHist[tick];
    field.map((row, i) => row.map((cell, j) => {
      if (cell === ">") {
        if (field[i][j - 1] !== ".") {
          nextField[i][j - 1] = '.';
          write(i, j + 1, field[i][j - 1]);
        }
      }
      if (cell === "<") {
        if (field[i][j + 1] !== ".") {
          nextField[i][j + 1] = '.';
          write(i, j - 1, field[i][j + 1]);
        }
      }
      if (cell === "^") {
        if (field[i + 1][j] !== ".") {
          nextField[i + 1][j] = '.';
          write(i - 1, j, field[i + 1][j]);
        }
      }
      if (cell === "v") {
        if (field[i - 1][j] !== ".") {
          nextField[i - 1][j] = '.';
          write(i + 1, j, field[i - 1][j]);
        }
      }
      if (cell === "+") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j])) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1] + field[i - 1][j]);
          write(i + 1, j, field[i][j - 1] + field[i - 1][j]);
        }
      }
      if (cell === "-") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j])) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1] - field[i - 1][j]);
          write(i + 1, j, field[i][j - 1] - field[i - 1][j]);
        }
      }
      if (cell === "*") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j])) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1] * field[i - 1][j]);
          write(i + 1, j, field[i][j - 1] * field[i - 1][j]);
        }
      }
      if (cell === "/") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j])) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1] / field[i - 1][j] | 0);
          write(i + 1, j, field[i][j - 1] / field[i - 1][j] | 0);
        }
      }
      if (cell === "%") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j])) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1] % field[i - 1][j]);
          write(i + 1, j, field[i][j - 1] % field[i - 1][j]);
        }
      }
      if (cell === "=") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j]) && field[i][j - 1] === field[i - 1][j]) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i][j - 1]);
          write(i + 1, j, field[i][j - 1]);
        }
      }
      if (cell === "#") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i - 1][j]) && field[i][j - 1] !== field[i - 1][j]) {
          nextField[i][j - 1] = '.';
          nextField[i - 1][j] = '.';
          write(i, j + 1, field[i - 1][j]);
          write(i + 1, j, field[i][j - 1]);
        }
      }
      if (cell === "@") {
        if (isNumber(field[i][j - 1]) && isNumber(field[i][j + 1]) && isNumber(field[i + 1][j]) && field[i - 1][j] !== ".") {
          stop = false;
          const di = field[i][j + 1];
          const dj = field[i][j - 1];
          const dt = field[i + 1][j];
          const v = field[i - 1][j];
          const wi = i - di;
          const wj = j - dj;
          const nextTick = tick - dt;
          if (nextTick < 0) {
            setError("Warp to negative tick");
            return;
          }
          if (warpTick !== null && warpTick !== nextTick) {
            setError("Multiple warps");
            return;
          }
          if (warpTick === null) {
            warpTick = nextTick;
            warpField = fieldHist[nextTick].map(row => row.map(cell => cell));
          }
          warpField[wi][wj] = v;
        }
      }
    }));
    writeField.map((row, i) => row.map((cell, j) => {
      if (cell !== null) {
        if (nextField[i][j] === "S") {
          if (answer !== null) {
            setError("Multiple answers");
          }
          setAnswer(cell);
          stop = true;
        }
        nextField[i][j] = cell;
      }
    }));
    if (!stop && warpTick !== null) {
      setTick(warpTick);
      setFieldHist([...fieldHist.slice(0, warpTick), warpField]);
      return;
    }
    setFieldHist([...fieldHist, nextField]);
    setTick(tick + 1);
    setTotalTick((t) => t + 1);
  }

  return (
    <div>
      <div>
        <h2>Values</h2>
        <div>
          <label>Val A</label>
          <input type="number" value={valA} onChange={onChangeValA} /><br />
          <label>Val B</label>
          <input type="number" value={valB} onChange={onChangeValB} />
        </div>
      </div>
      <div>
        <button onClick={nextTick} disabled={answer !== null && error === null}>Next</button>
        <button onClick={initialize}>Reset</button>
      </div>
      <div>T: {tick + 1}</div>
      <div>Tick: {totalTick}</div>
      <div>Answer: {answer !== null ? answer : "null"}</div>
      <div>Error: {error !== null ? error : "null"}</div>
      <Visualizer field={fieldHist[tick]} />
    </div>
  );
}
