import './App.css'
import { LambdaManEmulator } from './LambdaManEmulator'
import { CellType } from 'src/types';
import { useState, useEffect, useRef } from 'react'

function App() {
  const [field, setField] = useState<CellType[][]>([])
  const fieldRef = useRef<HTMLTextAreaElement>(null)

  useEffect(() => {
    if (field.length === 0) {
      return;
    }
    localStorage.setItem("field", JSON.stringify(field));
  }, [field]);

  useEffect(() => {
    const storedField = localStorage.getItem("field");
    if (storedField) {
      const parsedField = JSON.parse(storedField);
      setField(parsedField);
      if (fieldRef.current) {
        fieldRef.current.value = parsedField.map((line: CellType[]) => line.join("")).join("\n");
      }
    }
  }, []);


  return (
    <>
      <h1>Input field</h1>
      <textarea
        ref={fieldRef}
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
