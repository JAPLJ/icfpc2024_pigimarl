import 'src/App.css';
import { CellType } from 'src/types';

type VisualizerProps = {
  field: CellType[][];
  position: [number, number];
};

const Cell = (props: { cell: CellType, isLambdaManHere: boolean }) => {
  let backgroundColor = "white";
  let children = null;
  switch (props.cell) {
    case ".":
      backgroundColor = "white";
      children = ".";
      break;
    case "#":
      backgroundColor = "black";
      break;
    case "L":
      backgroundColor = "yellow";
      break;
    case " ":
      backgroundColor = "gray";
      break;
  }

  if (props.isLambdaManHere) {
    children = "o";
  }

  return <div className="cell" style={{ backgroundColor }}>{children}</div>;

};

export const Visualizer = (props: VisualizerProps) => {
  return (
    <div className="field">
      {props.field.map((row, rowIndex) => (
        <div className="row" key={rowIndex}>
          {row.map((cell, cellIndex) => (
            <Cell key={cellIndex} cell={cell} isLambdaManHere={rowIndex === props.position[0] && cellIndex === props.position[1]} />
          ))}
        </div>
      ))}
    </div>
  );
}
