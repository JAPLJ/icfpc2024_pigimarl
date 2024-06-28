import 'src/App.css';
import { Operator, CellType } from 'src/types';

type VisualizerProps = {
  field: CellType[][];
};

const Cell = (props: { cell: CellType }) => {
  let s = ".";
  if (typeof props.cell === "number") {
    s = props.cell.toString();
  } else if (typeof props.cell === "string") {
    s = props.cell;
  }
  return <div className="cell">{ s }</div>;
};

export const Visualizer = ({ field }: VisualizerProps) => {
  if (field === undefined || field.length === 0) {
    return <div>Empty field</div>;
  }
  return (
    <div className="field">
      {field.map((row, rowIndex) => (
        <div className="row" key={rowIndex}>
          {row.map(cell => (
            <Cell cell={cell} />
          ))}
        </div>
      ))}
    </div>
  );
}
