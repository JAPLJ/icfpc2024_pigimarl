import { Visualizer } from 'src/LambdaManEmulator/Visualizer';
import { useState, useEffect, useRef, useCallback } from 'react';
import { CellType } from 'src/types';
import { useInterval } from 'src/hooks/useInterval';

type LambdaManEmulatorProps = {
  field: CellType[][];
};

export const LambdaManEmulator = (props: LambdaManEmulatorProps) => {
  const [field, setField] = useState<CellType[][]>([]);
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
  const [animation, setAnimation] = useState<string>("");

  useEffect(() => {
    setPosition(findPosition(props.field));
    setField(props.field);
  }, [props.field]);

  const onMove: (x: number, y: number) => boolean = (x, y) => {
    if (x < 0 || x >= field.length || y < 0 || y >= field[0].length) {
      return false;
    }
    if (field[x][y] === "#") {
      return false;
    }
    if (field[x][y] === ".") {
      const newField = structuredClone(field);
      newField[x][y] = " ";
      setField(newField);
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

  const onReplay = useCallback(() => {
    setPosition(findPosition(props.field));
    setField(props.field);
    setAnimation(moves.join(""));
  }, [props.field, moves]);

  const animate = () => {
    if (animation.length === 0) {
      return;
    }
    const move = animation[0];
    setAnimation(animation.slice(1));
    switch (move) {
      case "U":
        onMove(position[0] - 1, position[1]);
        break;
      case "D":
        onMove(position[0] + 1, position[1]);
        break;
      case "L":
        onMove(position[0], position[1] - 1);
        break;
      case "R":
        onMove(position[0], position[1] + 1);
        break;
    }
  }
  const [delay, setDelay] = useInterval(animate, 1000);

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

  const onChangeMoves = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = event.target.value;
    setMoves(value.split(""));
  };

  return (
    <div onKeyUp={onKeyUp} autoFocus ref={ref} tabIndex={0}>
      <Visualizer field={field} position={position!} />
      <div>
        <h2>Moves</h2>
        <textarea className="moves" value={moves.join("")} disabled={animation.length > 0} onChange={onChangeMoves} />
        <div>
          <button onClick={() => navigator.clipboard.writeText(moves.join(""))}>Copy</button>
        </div>
        <div>
          <label>Animation Delay</label>
          <input type="range" min={0} max={1000} value={delay} onChange={(e) => setDelay(parseInt(e.target.value))} />
          <button onClick={onReplay}>Replay</button>
          <button onClick={() => setAnimation("")}>Stop</button>
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
