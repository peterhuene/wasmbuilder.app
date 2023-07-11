import { useRef, useState, useCallback } from "react";
import { ArrowDownOnSquareIcon, Bars3Icon } from "@heroicons/react/24/outline";
import ComponentLibrary from "./library";
import ReactFlow, {
  Node,
  MiniMap,
  Controls,
  Background,
  ReactFlowProvider,
  BackgroundVariant,
} from "reactflow";
import { Component, Instance, useAppState } from "./state";
import InstanceNode from "./nodes";
import colors from "tailwindcss/colors";
import { ComponentDetails } from "./details";
import NotificationPopup from "./notification";
import { DownloadComponentDialog } from "./dialogs";
import githubLogo from "./images/github.png";

const nodeTypes = {
  instance: InstanceNode,
};

const nodeColor = (node: Node<Instance>) => {
  const parts = node.data.component.color.split("-", 3);
  return colors[parts[1]][parts[2]];
};

const defaultEdgeOptions = { animated: true };

const onDragOver = (event: React.DragEvent<HTMLDivElement>) => {
  event.preventDefault();
  event.dataTransfer.dropEffect = "move";
};

const App = () => {
  const {
    nodes,
    edges,
    instantiateComponent,
    removeComponent,
    onNodesChange,
    onNodesDelete,
    onEdgesChange,
    onEdgesDelete,
    onConnect,
  } = useAppState();
  const componentLibrary = useRef(null);
  const reactFlowWrapper = useRef(null);
  const [reactFlowInstance, setReactFlowInstance] = useState(null);
  const [showDownloadComponentDialog, setShowDownloadComponentDialog] =
    useState(false);

  const onDrop = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault();

      const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
      const name = event.dataTransfer.getData("application/reactflow");

      if (typeof name === "undefined" || !name) {
        return;
      }

      const position = reactFlowInstance.project({
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      instantiateComponent(name, position);
    },
    [reactFlowInstance],
  );

  const onDownloadComponent = () => {
    setShowDownloadComponentDialog(true);
  };

  const onComponentDownloaded = (component: Component | null) => {
    setShowDownloadComponentDialog(false);

    if (!component) {
      return;
    }

    componentLibrary.current.addComponent(component);
  };

  return (
    <>
      <ComponentLibrary ref={componentLibrary} />
      <ComponentDetails onRemoveComponent={removeComponent} />
      <DownloadComponentDialog
        show={showDownloadComponentDialog}
        onClose={onComponentDownloaded}
        onSubmit={(name) => componentLibrary.current.checkComponentName(name)}
      />
      <div className="flex flex-1 flex-col md:pl-80">
        <div className="sticky top-0 z-10 bg-gray-800 pl-1 pt-1">
          <div className="flex w-full items-center justify-between">
            <button
              type="button"
              className="md:hidden flex -ml-0.5 -mt-0.5 inline-flex h-12 w-12 items-center justify-center rounded-md text-white hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
              onClick={() => componentLibrary.current.showSidebar()}
            >
              <span className="sr-only">Open sidebar</span>
              <Bars3Icon className="h-6 w-6" aria-hidden="true" />
            </button>
            <button
              type="button"
              disabled={nodes.length === 0}
              className="my-2 ml-auto mr-4 inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 disabled:bg-indigo-900 disabled:text-gray-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-800"
              onClick={onDownloadComponent}
            >
              <ArrowDownOnSquareIcon className="mr-1 h-5 w-5" />
              <span>Download Component</span>
            </button>
            <a
              href="https://github.com/peterhuene/wasmbuilder.app"
              target="_blank"
              rel="noreferrer"
              className="my-2 mr-4 inline-flex items-center justify-center"
            >
              <img src={githubLogo}></img>
            </a>
          </div>
        </div>
        <main>
          <div
            ref={reactFlowWrapper}
            className="absolute left-0 w-full h-[calc(100%-3.5rem)] md:left-80 top-14 md:w-[calc(100%-20rem)]"
          >
            <ReactFlowProvider>
              <ReactFlow
                className=""
                nodeTypes={nodeTypes}
                nodes={nodes}
                edges={edges}
                onInit={setReactFlowInstance}
                onDragOver={onDragOver}
                onDrop={onDrop}
                onNodesChange={onNodesChange}
                onNodesDelete={onNodesDelete}
                onEdgesChange={onEdgesChange}
                onEdgesDelete={onEdgesDelete}
                onConnect={onConnect}
                defaultEdgeOptions={defaultEdgeOptions}
              >
                <Background
                  variant={BackgroundVariant.Dots}
                  size={1}
                  gap={15}
                  color="black"
                />
                <MiniMap
                  className="border border-gray-300"
                  nodeColor={nodeColor}
                />
                <Controls />
              </ReactFlow>
            </ReactFlowProvider>
          </div>
        </main>
      </div>
      <NotificationPopup />
    </>
  );
};

export default App;
