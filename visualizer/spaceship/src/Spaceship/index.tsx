import { Point } from 'src/types'
import { ScatterPlot } from 'src/Spaceship/ScatterPlot'

type SpaceshipProps = {
  points: Point[];
};

export const Spaceship = ({ points }: SpaceshipProps) => {

  return <div className="spaceship">
    <ScatterPlot points={points} />
  </div>
};

