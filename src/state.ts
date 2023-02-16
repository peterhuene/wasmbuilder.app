import {
  Node,
  Edge,
  XYPosition,
  NodeChange,
  applyNodeChanges,
  applyEdgeChanges,
  EdgeChange,
  Connection,
} from "reactflow";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { Component as GraphComponent } from "./exports/graph";
import { graph } from "./graph";

export type Component = GraphComponent & {
  description: string;
  color: string;
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
  exportedInstance: Instance | null;
  notifications: Notification[];
  addComponent: (component: Component) => void;
  removeComponent: (component: Component) => void;
  selectComponent: (component: Component | null) => void;
  exportInstance: (instance: Instance | null) => void;
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
        graph.removeComponent(component.id);
        set((state) => {
          if (state.exportedInstance?.component.id == component.id) {
            state.exportedInstance = null;
          }
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
      exportInstance: (instance) => {
        set((state) => {
          state.exportedInstance = instance;

          if (instance) {
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
          const id = graph.instantiateComponent(component.id);
          state.nodes.push({
            id: id.toString(),
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
      onNodesDelete: (deleted: Node<Instance>[]) => {
        set((state) => {
          deleted.forEach((node) => {
            if (state.exportedInstance?.id === node.data.id) {
              state.exportedInstance = null;
            }
            graph.removeInstance(node.data.id);
          });
        });
      },
      onEdgesChange: (changes: EdgeChange[]) => {
        set((state) => {
          state.edges = applyEdgeChanges(changes, state.edges);
        });
      },
      onEdgesDelete: (deleted: Edge[]) => {
        deleted.forEach((edge) => {
          graph.disconnectInstances(
            Number(edge.source),
            Number(edge.target),
            Number(edge.targetHandle)
          );
        });
      },
      onConnect: (connection: Edge | Connection) => {
        try {
          graph.connectInstances(
            Number(connection.source),
            connection.sourceHandle === "i"
              ? null
              : Number(connection.sourceHandle),
            Number(connection.target),
            Number(connection.targetHandle)
          );
        } catch (e) {
          set((state) => {
            state.notifications.push({
              type: NotificationType.Error,
              title: "Invalid connection",
              message: e.payload,
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
