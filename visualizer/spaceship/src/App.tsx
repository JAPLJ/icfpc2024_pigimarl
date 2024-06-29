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

function App() {
  const [points, setPoints] = useState<Point[]>();
  const fieldRef = useRef<HTMLTextAreaElement>(null);

  return (
    <>
      <h1>Spaceship</h1>
      <textarea
        ref={fieldRef}
        className="input"
        onChange={(e) => {
          const f = e.target.value
          setPoints(parse(f))
        }}
      />
      <Spaceship points={points || []} />
    </>
  )
}

export default App
