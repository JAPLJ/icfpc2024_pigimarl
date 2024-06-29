import './App.css'
import { Token } from 'src/types'
import { ICFPTree } from 'src/ICFPTree'
import { Parser } from 'src/Parser'
import { useState, useEffect, useRef } from 'react'

function App() {
  const [tree, setTree] = useState<Token[]>();
  const fieldRef = useRef<HTMLTextAreaElement>(null);
  const parser = new Parser();

  // useEffect(() => {
  //   if (fieldRef.current) {
  //     // fieldRef.current.value = 'B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L% ? B< v% I# I" B+ B$ v$ B- v% I" B$ v$ B- v% I# II';
  //     // fieldRef.current.value = '? B> I# I$ S9%3 S./';
  //     // fieldRef.current.value = 'I/6';
  //   }
  // }, []);

  return (
    <>
      <h1>ICFP</h1>
      <textarea
        ref={fieldRef}
        className="input"
        onChange={(e) => {
          const f = e.target.value
          setTree(parser.parse(f))
        }}
      />
      <div className="output">
        <ICFPTree tree={tree} />
      </div>
    </>
  )
}

export default App
