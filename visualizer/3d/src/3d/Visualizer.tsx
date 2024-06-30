import 'src/App.css';
import { Operator, CellType } from 'src/types';

type VisualizerProps = {
  field: CellType[][];
};

const Cell = (props: { cell: CellType, i: number, j: number }) => {
  let s = ".";
  if (typeof props.cell === "number") {
    s = props.cell.toString();
  } else if (typeof props.cell === "string") {
    s = props.cell;
  }
  return <div className="cell">{s}
    <span className="tooltip">({props.i}, {props.j}) {s}</span>
  </div>;
};

export const Visualizer = ({ field }: VisualizerProps) => {
  if (field === undefined || field.length === 0) {
    return <div>Empty field</div>;
  }
  return (
    <div className="field">
      {field.map((row, i) => (
        <div className="row" key={i}>
          {row.map((cell, j) => (
            <Cell cell={cell} i={i} j={j} key={j} />
          ))}
        </div>
      ))}
    </div>
  );
}
