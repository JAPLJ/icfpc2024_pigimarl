import { useState, useEffect } from 'react'
import { Point } from 'src/types'
// import { ScatterPlot } from 'src/Spaceship/ScatterPlot'
import { ScatterPlot } from 'src/Spaceship/LightScatterPlot'
import { useInterval } from 'src/hooks/useInterval'

type SpaceshipProps = {
  points: Point[];
  trails?: Point[];
};

export const Spaceship = ({ points, trails }: SpaceshipProps) => {
  const [isAnimating, setIsAnimating] = useState<boolean>(false);
  const [animateIndex, setAnimateIndex] = useState<number>(0);
  const [animeTrails, setAnimeTrails] = useState<Point[]>(trails ?? []);
  const onAnimate = () => {
    setIsAnimating(!isAnimating);
  }
  useInterval(() => {
    if (isAnimating) {
      setAnimateIndex((animateIndex + 1) % (trails?.length ?? 1));
    }
  }, 1000);
  useEffect(() => {
    if (!trails) return;
    setAnimeTrails(trails);
  }, [trails]);
  useEffect(() => {
    if (!trails) return;
    setAnimeTrails(trails!.slice(0, animateIndex + 1));
  }, [animateIndex]);

  return <div className="spaceship">
    <ScatterPlot points={points} trails={animeTrails}/>
    <button onClick={onAnimate}>Start</button>
    <input type="range" min="0" max={trails?.length} value={animateIndex} onChange={(e) => setAnimateIndex(Number(e.target.value))}/>
  </div>
};

