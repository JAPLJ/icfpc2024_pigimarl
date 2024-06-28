import { Visualizer } from 'src/LambdaManEmulator/Visualizer';
import { useState, useEffect, useRef } from 'react';
import { CellType } from 'src/types';

type LambdaManEmulatorProps = {
  field: CellType[][];
};

export const LambdaManEmulator = ({ field }: LambdaManEmulatorProps) => {
  const findPosition: (field: CellType[][]) => [number, number] = (field) => {
    for (let i = 0; i < field.length; i++) {
      for (let j = 0; j < field[i].length; j++) {
        if (field[i][j] === "L") {
          return [i, j];
        }
      }
    }
    return [0, 0];
  };
  const [position, setPosition] = useState<[number, number]>([0, 0]);
  const [moves, setMoves] = useState<string[]>([]);
  const [problemNumber, setProblemNumber] = useState<number>(0);

  useEffect(() => {
    setPosition(findPosition(field));
  }, [field]);

  const onMove: (x: number, y: number) => boolean = (x, y) => {
    if (x < 0 || x >= field.length || y < 0 || y >= field[0].length) {
      return false;
    }
    if (field[x][y] === "#") {
      return false;
    }
    if (field[x][y] === ".") {
      field[x][y] = " ";
    }
    setPosition([x, y]);
    return true;
  };

  const onKeyUp = (event: React.KeyboardEvent) => {
    let result = false;
    let move = ""
    switch (event.key) {
      case "ArrowUp":
        result = onMove(position[0] - 1, position[1]);
        move = "U"
      break;
      case "ArrowDown":
        result = onMove(position[0] + 1, position[1]);
        move = "D"
      break;
      case "ArrowLeft":
        result = onMove(position[0], position[1] - 1);
        move = "L"
      break;
      case "ArrowRight":
        result = onMove(position[0], position[1] + 1);
        move = "R"
      break;
    }
    if (result) {
      setMoves([...moves, move]);
    }
  };

  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    ref.current?.focus();
  }, [ref]);

  const problemNumberRef = useRef<HTMLInputElement>(null);
  useEffect(() => {
    if (problemNumberRef.current && problemNumber) {
      localStorage.setItem("problemNumber", problemNumber.toString());
    }
  }, [problemNumber]);

  useEffect(() => {
    const storedProblemNumber = localStorage.getItem("problemNumber");
    if (storedProblemNumber) {
      setProblemNumber(parseInt(storedProblemNumber));
      problemNumberRef.current!.value = storedProblemNumber;
    }
  }, []);

  return (
    <div onKeyUp={onKeyUp} autoFocus ref={ref} tabIndex={0}>
      <Visualizer field={field} position={position!} />
      <div>
        <h2>Moves</h2>
        <textarea className="moves" value={moves.join("")} readOnly />
        <div>
          <button onClick={() => navigator.clipboard.writeText(moves.join(""))}>Copy</button>
        </div>
        <div>
          <label>Problem Number</label>
          <input ref={problemNumberRef} className="problemNumber" type="number" value={problemNumber} onChange={(e) => setProblemNumber(parseInt(e.target.value))} />
        </div>
        <a href={`http://localhost:8080/solve%20lambdaman${problemNumber}%20${moves.join("")}`} target={"blank"} >submit</a>
      </div>
    </div>
  );
}
