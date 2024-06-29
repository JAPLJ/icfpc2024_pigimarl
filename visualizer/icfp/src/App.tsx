import './App.css'
import { Token, Tree } from 'src/types'
import { ICFPTree } from 'src/ICFPTree'
import { Parser } from 'src/Parser'
import { useState, useRef } from 'react'

function App() {
  const [tree, setTree] = useState<Tree>();
  const fieldRef = useRef<HTMLTextAreaElement>(null);
  const parser = new Parser();

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
        <ICFPTree tree={tree!} />
      </div>
    </>
  )
}

export default App
