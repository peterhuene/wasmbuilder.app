import {
  Node,
  Edge,
  XYPosition,
  NodeChange,
  applyNodeChanges,
  applyEdgeChanges,
  EdgeChange,
  Connection,
} from "react-flow-renderer";
import create from "zustand";
import { immer } from "zustand/middleware/immer";
import {
  instantiateComponent,
  removeInstance,
  removeComponent,
  connectInstances,
  disconnectInstances,
} from "./graph";

export type Import = {
  index: number;
  name: string;
  kind: string;
};

export type Export = {
  name: string;
  kind: string;
};

export type Component = {
  id: number;
  name: string;
  description: string;
  color: string;
  interface: string | null;
  imports: Import[];
  exports: Export[];
};

export type Instance = {
  id: number;
  component: Component;
};

export enum NotificationType {
  Success,
  Error,
}

export type Notification = {
  type: NotificationType;
  title: string;
  message: string;
};

interface AppState {
  components: Record<string, Component>;
  nodes: Node<Instance>[];
  edges: Edge[];
  selectedComponent: Component | null;
  exportedInstance: number | null;
  notifications: Notification[];
  addComponent: (component: Component) => void;
  removeComponent: (component: Component) => void;
  selectComponent: (component: Component | null) => void;
  exportInstance: (instance: number | null) => void;
  instantiateComponent: (name: string, position: XYPosition) => void;
  onNodesChange: (changes: NodeChange[]) => void;
  onNodesDelete: (nodes: Node[]) => void;
  onEdgesChange: (changes: EdgeChange[]) => void;
  onEdgesDelete: (edges: Edge[]) => void;
  onConnect: (connection: Edge | Connection) => void;
  pushNotification: (notification: Notification) => void;
  popNotification: () => void;
}

export const useAppState = create<AppState>()(
  immer((set) => {
    return {
      nodes: [],
      edges: [],
      components: {},
      selectedComponent: null,
      exportedInstance: null,
      notification: null,
      notifications: [],
      addComponent: (component) => {
        set((state) => {
          const components = state.components;
          components[component.name] = component;
        });
      },
      removeComponent: (component) => {
        removeComponent(component.id);
        set((state) => {
          delete state.components[component.name];
          state.nodes = state.nodes.filter(
            (node) => node.data.component.id !== component.id
          );
        });
      },
      selectComponent: (component) => {
        set((state) => {
          state.selectedComponent = component;
        });
      },
      exportInstance: (id) => {
        set((state) => {
          state.exportedInstance = id;

          if (id) {
            state.notifications.push({
              type: NotificationType.Success,
              title: "Exporting instance",
              message:
                "The instance's exports will be exported from the component",
            });
          }
        });
      },
      instantiateComponent: (name, position) => {
        set((state) => {
          const component = state.components[name];
          const id = instantiateComponent(component.id).toString();
          state.nodes.push({
            id,
            type: "instance",
            dragHandle: ".node-header",
            position,
            data: {
              id,
              component,
            },
          });
        });
      },
      onNodesChange: (changes: NodeChange[]) => {
        set((state) => {
          state.nodes = applyNodeChanges(changes, state.nodes);
        });
      },
      onNodesDelete: (deleted: Node[]) => {
        deleted.forEach((node) => {
          removeInstance(node.id);
        });
      },
      onEdgesChange: (changes: EdgeChange[]) => {
        set((state) => {
          state.edges = applyEdgeChanges(changes, state.edges);
        });
      },
      onEdgesDelete: (deleted: Edge[]) => {
        deleted.forEach((edge) => {
          disconnectInstances(edge.source, edge.target, edge.targetHandle);
        });
      },
      onConnect: (connection: Edge | Connection) => {
        try {
          connectInstances(
            connection.source,
            connection.sourceHandle === "i" ? null : connection.sourceHandle,
            connection.target,
            connection.targetHandle
          );
        } catch (e) {
          if (e instanceof Error) {
            throw e;
          }

          set((state) => {
            state.notifications.push({
              type: NotificationType.Error,
              title: "Invalid connection",
              message: e,
            });
          });
          return;
        }

        set((state) => {
          const exp =
            connection.sourceHandle === "i"
              ? "$instance"
              : state.nodes.find((n) => n.id === connection.source).data
                  .component.exports[connection.sourceHandle].name;
          const imp = state.nodes.find((n) => n.id === connection.target).data
            .component.imports[connection.targetHandle].name;

          state.edges.push({
            ...connection,
            id: `${connection.source}:${connection.sourceHandle}->${connection.target}:${connection.targetHandle}`,
            label: `${exp} → ${imp}`,
          });
        });
      },
      pushNotification(notification) {
        set((state) => {
          state.notifications.push(notification);
        });
      },
      popNotification() {
        set((state) => {
          state.notifications = state.notifications.slice(1);
        });
      },
    };
  })
);
