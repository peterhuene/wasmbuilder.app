import React, {
  Fragment,
  useState,
  useImperativeHandle,
  Ref,
  DragEvent,
  DragEventHandler,
  MouseEventHandler,
} from "react";
import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { PlusIcon, PuzzlePieceIcon } from "@heroicons/react/24/solid";
import { AddComponentDialog } from "./dialogs";
import { Component, useAppState } from "./state";

type AddComponentButtonProps = {
  onClick: () => void;
};

const AddComponentButton = ({ onClick }: AddComponentButtonProps) => {
  return (
    <a className="group block w-full flex-shrink-0">
      <div className="flex items-center">
        <button
          type="button"
          onClick={onClick}
          className="relative inline-flex grow items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-800"
        >
          <PlusIcon className="mr-1 h-5 w-5" />
          <span>Add Component</span>
        </button>
      </div>
    </a>
  );
};

type ComponentListItemProps = {
  component: Component;
  onItemClick: MouseEventHandler;
  onItemDragStart: DragEventHandler;
};

const ComponentListItem = ({
  component,
  onItemClick,
  onItemDragStart,
}: ComponentListItemProps) => {
  const parts = component.color.split("-", 3);
  const borderColor = "border-" + parts[1] + "-600";
  const hoverColor = "hover:bg-" + parts[1] + "-100";
  return (
    <li
      className={`relative py-5 px-4 mr-3 ml-3 mb-3 rounded-lg cursor-pointer border ${component.color} ${borderColor} ${hoverColor}`}
      draggable={true}
      onDragStart={onItemDragStart}
      onClick={onItemClick}
    >
      <div className="flex justify-between space-x-3">
        <div className="flex-shrink-0">
          <PuzzlePieceIcon className="h-8 w-8 text-indigo-900" />
        </div>
        <div className="min-w-0 flex-1 inline-flex items-center">
          <span className="absolute inset-0" aria-hidden="true" />
          <p className="truncate text-sm font-medium text-gray-900">
            {component.name}
          </p>
        </div>
      </div>
    </li>
  );
};

type ComponentListProps = {
  onAddClick: () => void;
  onItemClick: (component: Component) => void;
  onDragItem?: (component: Component) => void;
};

const ComponentList = ({
  onAddClick,
  onItemClick,
  onDragItem,
}: ComponentListProps) => {
  const { components } = useAppState();

  if (Object.keys(components).length === 0) {
    return (
      <div className="flex h-screen text-white">
        <div className="m-auto text-center">
          <p>There are no components in the library</p>
          <p>
            <a className="cursor-pointer" onClick={onAddClick}>
              <u>Add a component</u>
            </a>
          </p>
        </div>
      </div>
    );
  }

  const handleDragStart = (event: DragEvent<Element>, component: Component) => {
    if (onDragItem) {
      onDragItem(component);
    }

    event.dataTransfer.setData("application/reactflow", component.name);
    event.dataTransfer.effectAllowed = "move";
  };

  return (
    <div className="h-0 flex-1 overflow-y-auto pt-14 pb-4">
      <ul role="list" className="select-none">
        {Object.entries(components).map(([, component]) => (
          <ComponentListItem
            key={component.id}
            component={component}
            onItemClick={() => onItemClick(component)}
            onItemDragStart={(e) => handleDragStart(e, component)}
          />
        ))}
      </ul>
    </div>
  );
};

type SidebarProps = {
  show: boolean;
  onClose: (show: boolean) => void;
  onAddClick: () => void;
  onItemClick: (component: Component) => void;
};

const Sidebar = ({ show, onClose, onAddClick, onItemClick }: SidebarProps) => {
  return (
    <Transition.Root show={show} as={Fragment}>
      <Dialog as="div" className="relative z-40 md:hidden" onClose={onClose}>
        <Transition.Child
          as={Fragment}
          enter="transition-opacity ease-linear duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="transition-opacity ease-linear duration-300"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
        </Transition.Child>

        <div className="fixed inset-0 z-40 flex">
          <Transition.Child
            as={Fragment}
            enter="transition ease-in-out duration-300 transform"
            enterFrom="-translate-x-full"
            enterTo="translate-x-0"
            leave="transition ease-in-out duration-300 transform"
            leaveFrom="translate-x-0"
            leaveTo="-translate-x-full"
          >
            <Dialog.Panel className="relative flex w-full max-w-xs flex-1 flex-col bg-gray-800">
              <Transition.Child
                as={Fragment}
                enter="ease-in-out duration-300"
                enterFrom="opacity-0"
                enterTo="opacity-100"
                leave="ease-in-out duration-300"
                leaveFrom="opacity-100"
                leaveTo="opacity-0"
              >
                <div className="absolute top-0 right-0 -mr-10 pt-4">
                  <button
                    type="button"
                    className="rounded-md text-gray-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-white"
                    onClick={() => onClose(false)}
                  >
                    <span className="sr-only">Close panel</span>
                    <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                  </button>
                </div>
              </Transition.Child>
              <ComponentList
                onAddClick={onAddClick}
                onItemClick={onItemClick}
                onDragItem={() => onClose(false)}
              />
              <div className="flex flex-shrink-0 bg-gray-700 p-4">
                <AddComponentButton onClick={onAddClick} />
              </div>
            </Dialog.Panel>
          </Transition.Child>
          <div className="w-14 flex-shrink-0">
            {/* Force sidebar to shrink to fit close icon */}
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};

type ComponentLibraryRef = {
  showSidebar: () => void;
  checkComponentName: (name: string) => string;
  addComponent: (component: Component) => void;
};

const ComponentLibrary = React.forwardRef(
  (_: unknown, ref: Ref<ComponentLibraryRef>) => {
    const { components, addComponent, selectComponent } = useAppState();
    const [showSidebar, setShowSidebar] = useState(false);
    const [showAddComponentDialog, setShowAddComponentDialog] = useState(false);

    const checkComponentName = (name: string) => {
      if (name in components) {
        return "A component with name '" + name + "' already exists.";
      }
      return "";
    };

    useImperativeHandle(ref, () => ({
      showSidebar: () => setShowSidebar(true),
      checkComponentName,
      addComponent: (component: Component) => {
        addComponent(component);
      },
    }));

    const handleAddClick = () => {
      setShowSidebar(false);
      setShowAddComponentDialog(true);
    };

    const handleItemClick = (item) => {
      setShowSidebar(false);
      selectComponent(item);
    };

    const handleAddDialogClose = (component) => {
      setShowAddComponentDialog(false);
      if (component) {
        addComponent(component);
      }
    };

    return (
      <>
        <AddComponentDialog
          show={showAddComponentDialog}
          onClose={handleAddDialogClose}
          onSubmit={checkComponentName}
        />
        <Sidebar
          show={showSidebar}
          onClose={setShowSidebar}
          onAddClick={handleAddClick}
          onItemClick={handleItemClick}
        />
        <div className="hidden md:fixed md:inset-y-0 md:flex md:w-80 md:flex-col flex min-h-0 flex-1 flex-col bg-gray-800">
          <ComponentList
            onAddClick={handleAddClick}
            onItemClick={handleItemClick}
          />
          <div className="flex flex-shrink-0 bg-gray-700 p-4">
            <AddComponentButton onClick={handleAddClick} />
          </div>
        </div>
      </>
    );
  }
);

ComponentLibrary.displayName = "Component Library";

export default ComponentLibrary;
