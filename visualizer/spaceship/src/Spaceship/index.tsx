import { Point } from 'src/types'
// import { ScatterPlot } from 'src/Spaceship/ScatterPlot'
import { ScatterPlot } from 'src/Spaceship/LightScatterPlot'

type SpaceshipProps = {
  points: Point[];
  trails?: Point[];
};

export const Spaceship = ({ points, trails }: SpaceshipProps) => {

  return <div className="spaceship">
    <ScatterPlot points={points} trails={trails}/>
  </div>
};

