import './App.css'
import { useState, useRef } from 'react'
import { Point } from 'src/types'
import { Spaceship } from 'src/Spaceship'

const parse: (field: string) => Point[] = (field) => {
  const lines = field.split('\n').filter((line) => line.trim() !== '')
  const points = lines.map((line) => {
    const [x, y] = line.split(' ').map(Number)
    return [x, y] as Point;
  })
  return points
}

const calcTrail: (moves: number[]) => Point[] = (moves) => {
  let dx = 0, dy = 0, x = 0, y = 0;
  const trails: Point[] = [[x, y]];
  for(let i = 0; i < moves.length; i++) {
    const move = moves[i];
    if ([7, 8, 9].includes(move)) {
      dy += 1;
    }
    if ([1, 2, 3].includes(move)) {
      dy -= 1;
    }
    if ([1, 4, 7].includes(move)) {
      dx -= 1;
    }
    if ([3, 6, 9].includes(move)) {
      dx += 1;
    }
    x += dx;
    y += dy;
    trails.push([x, y]);
  }
  return trails;
}

function App() {
  const [points, setPoints] = useState<Point[]>();
  const [trails, setTrails] = useState<Point[]>();
  const fieldRef = useRef<HTMLTextAreaElement>(null);
  const moveRef = useRef<HTMLTextAreaElement>(null);

  return (
    <>
      <h1>Spaceship</h1>
      <div className="wrapper">
        <div>
          <h2>Field</h2>
          <textarea
            ref={fieldRef}
            className="input"
            onChange={(e) => {
              const f = e.target.value
              setPoints(parse(f))
            }}
          />
        </div>
        <div>
          <h2>Move</h2>
          <textarea
            ref={moveRef}
            className="input"
            onChange={(e) => {
              const f = e.target.value;
              const moves = f.split('').filter((c) => c !== '\n').map(Number);
              setTrails(calcTrail(moves));
            }}
          />
        </div>
      </div>
      <Spaceship points={points || []} trails={trails} />
    </>
  )
}

export default App
