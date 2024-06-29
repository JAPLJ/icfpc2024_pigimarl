import { Tree } from "src/types";

type ICFPTreeProps = {
  tree: Tree;
};

export const ICFPTree = ({ tree }: ICFPTreeProps) => {
  if (!tree) {
    return <div />;
  }
  return (
    <div className={`tree ${tree.type}`}>
      <div className="value">
        {tree?.value}
      </div>
      <div className="nodes">
        {tree?.nodes?.map((node, i) => (
          <ICFPTree key={i} tree={node} />
        ))}
      </div>
    </div>
  );
}

