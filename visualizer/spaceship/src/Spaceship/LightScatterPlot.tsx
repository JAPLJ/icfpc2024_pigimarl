import { useMemo } from 'react';
import Gradient from "javascript-color-gradient";
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

type SpaceshipProps = {
  points: Point[];
  trails?: Point[];
}

export const ScatterPlot = ({ points, trails }: SpaceshipProps) => {
  ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);
  const gradient = useMemo(() => {
    const numColors = trails?.length;
    const g = new Gradient();
    g.setColorGradient("#3F2CAF", "e9446a");
    g.setMidpoint(numColors ?? 1);
    return g.getColors();
  }, [trails]);

  const dataSets = [
    {
      label: 'Points',
      data: points.map(([x, y]) => ({ x, y })),
      backgroundColor: '#fff',
      pointRadius: 4,
      order: 3,
    },
    {
      label: 'original',
      data: [[0, 0]],
      backgroundColor: 'rgba(54, 162, 235, 1)',
      pointRadius: 4,
      order: 2,
    },
  ];
  if (trails && trails.length > 0) {
    dataSets.push({
      label: 'Trails',
      data: trails.map(([x, y]) => ({ x, y })),
      backgroundColor: gradient,
      pointStyle: 'triangle',
      pointRadius: 4,
      order: 1,
    });
  }
  return <Scatter
    data={{ datasets: dataSets }}
    options={{
      animation: false,
    }}
  />
};

