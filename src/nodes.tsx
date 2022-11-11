import { Handle, Position } from "react-flow-renderer";
import { Instance, useAppState } from "./state";

type InstanceNodeProps = {
  data: Instance;
  selected: boolean;
};

const InstanceNode = ({ data, selected }: InstanceNodeProps) => {
  const { exportedInstance, exportInstance } = useAppState();
  const color = data.component.color.split("-", 3)[1];

  let selectedClass = "";
  if (selected) {
    selectedClass = `outline-none ring-1 ring-${color}-600 ring-offset-1 ring-offset-${color}-700`;
  }

  return (
    <>
      <div
        className={`pb-1 flex ${data.component.color} border border-${color}-600 rounded-lg ${selectedClass}`}
      >
        <div className="flex flex-col">
          <div
            className={`node-header flex px-0.5 py-1 bg-${color}-500 rounded-t text-sm text-gray-100 text-center`}
          >
            <div className="w-1/5">
              <div className="flex h-5 items-center">
                <input
                  id="exported"
                  name="exported"
                  type="checkbox"
                  checked={data.id == exportedInstance?.id}
                  onChange={(e) =>
                    exportInstance(e.target.checked ? data : null)
                  }
                  className={`h-4 w-4 rounded border-${color}-300 text-${color}-500 focus:ring-0`}
                />
              </div>
            </div>
            <div className="w-4/5">{data.component.name}</div>
            <div className="w-1/5">
              <Handle
                type="source"
                className={`w-1/5 float-right mt-0.5 overflow-hidden bg-${color}-700`}
                position={Position.Right}
                id="i"
              >
                <span className="pointer-events-none align-top font-mono text-white text-xs">
                  I
                </span>
              </Handle>
            </div>
          </div>
          <div className="pt-1 flex flex-row min-w-[200px] min-h-[100px]">
            <div className="flex flex-col mr-6">
              {data.component.imports.map((imp, i) => (
                <div className="flex flex-row my-0.5 items-center" key={i}>
                  <Handle
                    type="target"
                    className={`text-center overflow-hidden ml-0.5 bg-${color}-700 rounded-sm`}
                    position={Position.Left}
                    id={i.toString()}
                  >
                    <span className="pointer-events-none align-top font-mono text-white text-xs">
                      {imp.kind[0].toUpperCase()}
                    </span>
                  </Handle>
                  <div className="ml-1 text-gray-900">{imp.name}</div>
                </div>
              ))}
            </div>
            <div className="ml-auto flex flex-col">
              {data.component.exports.map((exp, i) => (
                <div className="flex flex-row my-0.5 items-center" key={i}>
                  <div className="mr-1 text-gray-900">{exp.name}</div>
                  <Handle
                    type="source"
                    className={`text-center overflow-hidden ml-auto mr-0.5 bg-${color}-700`}
                    position={Position.Right}
                    id={i.toString()}
                  >
                    <span className="pointer-events-none align-top font-mono text-white text-xs">
                      {exp.kind[0].toUpperCase()}
                    </span>
                  </Handle>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default InstanceNode;
