import './App.css'
import { LambdaManEmulator } from './LambdaManEmulator'
import { CellType } from 'src/types';
import { useState } from 'react';

function App() {
  const [field, setField] = useState<CellType[][]>([])
  return (
    <>
      <h1>Input field</h1>
      <textarea
        className="filedInput"
        onChange={(e) => {
          const f = e.target.value
          setField(f.split('\n').map((line) => line.split('') as CellType[]))
        }}
      />

      <LambdaManEmulator field={field} />
    </>
  )
}

export default App
