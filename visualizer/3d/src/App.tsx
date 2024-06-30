import './App.css'
import { ThreeDEmulator } from './3d'
import { CellType } from 'src/types';
import { useState } from 'react'

function App() {
  const [initialField, setInitialField] = useState<CellType[][]>([])

  return (
    <>
      <h2>initial field</h2>
      <textarea
        className="fieldInput"
        onChange={(e) => {
          const f = e.target.value
          setInitialField(f.split('\n').map((line) => line.split(/\s+/) as CellType[]).filter((line) => line.length > 0))
        }}
      />

      <ThreeDEmulator initialField={initialField} />
    </>
  )
}

export default App
