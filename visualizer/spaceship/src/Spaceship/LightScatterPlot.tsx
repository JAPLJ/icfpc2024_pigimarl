import { Point } from 'src/types'
import {
  Chart as ChartJS,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
} from 'chart.js';
import { Scatter } from 'react-chartjs-2';

export const ScatterPlot = ({ points }: { points: Point[] }) => {
  ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);

  const dataSets = [
    {
      label: 'Points',
      data: points.map(([x, y]) => ({ x, y })),
      backgroundColor: 'rgba(255, 99, 132, 1)',
    },
    {
      label: 'original',
      data: [[0, 0]],
      backgroundColor: 'rgba(54, 162, 235, 1)',
    },
  ];
  return <Scatter
    data={{ datasets: dataSets }}
    options={{
      animation: false,
    }}
  />
};

