import { Point } from 'src/types'
import Chart from 'react-apexcharts'

export const ScatterPlot = ({ points }: { points: Point[] }) => {
  console.log(points)
  return (<Chart
    options={{
      chart: {
        type: 'scatter',
        zoom: {
          enabled: true,
          type: 'xy'
        }
      },
      xaxis: {
        tickAmount: 10,
        labels: {
          formatter: (val) => parseFloat(val).toFixed(1)
        }
      },
      yaxis: {
        tickAmount: 7
      },
      theme: {
        mode: 'dark',
      },
    }}
    series={[{
      name: 'Points',
      data: [...points],
    }, {
      name: "original",
      data: [[0, 0]],
    }]}
    type="scatter"
    width={800}
    height={800}
  />);
};

