import { Dialog, Listbox, Transition } from "@headlessui/react";
import { Fragment, useState, useRef, FormEvent } from "react";
import {
  ChevronDoubleDownIcon,
  ExclamationTriangleIcon,
  ExclamationCircleIcon,
} from "@heroicons/react/24/outline";
import { Component, useAppState, NotificationType, Graph } from "./state";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";

const Colors = [
  "slate",
  "orange",
  "amber",
  "emerald",
  "teal",
  "cyan",
  "sky",
  "blue",
  "indigo",
  "violet",
  "purple",
  "fuchsia",
  "rose",
]
  .map((color) => {
    return [`bg-${color}-200`, `bg-${color}-300`, `bg-${color}-400`];
  })
  .flat();

const classNames = (...classes: string[]) => {
  return classes.filter(Boolean).join(" ");
};

const randomColor = () => {
  return Colors[Math.floor(Math.random() * Colors.length)];
};

const downloadFile = (name: string, bytes: Uint8Array) => {
  const blob = new Blob([bytes], { type: "application/wasm" });
  const link = document.createElement("a");
  link.href = window.URL.createObjectURL(blob);
  if (!name.endsWith(".wasm")) {
    name += ".wasm";
  }
  link.download = name;
  link.click();
};

type AddComponentDialogProps = {
  show: boolean;
  onClose: (component: Component | null) => void;
  onSubmit: (name: string) => string;
};

export const AddComponentDialog = ({
  show,
  onClose,
  onSubmit,
}: AddComponentDialogProps) => {
  const nameInputRef = useRef<HTMLInputElement>(null);
  const [nameError, setNameError] = useState("");
  const descriptionInputRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File>(null);
  const [fileError, setFileError] = useState("");
  const [selectedColor, setSelectedColor] = useState(randomColor);

  const handleAfterLeave = () => {
    setNameError("");
    setFileError("");
    setFile(null);
    setSelectedColor(randomColor());
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    let nameError = "";
    let fileError = "";
    const name = nameInputRef.current?.value ?? "";
    const description = descriptionInputRef?.current.value ?? "";

    if (name.length === 0) {
      nameError = "Name is required";
      nameInputRef.current.focus();
    } else {
      nameError = onSubmit(name);
    }

    if (!file) {
      fileError = "WebAssembly component required";
    }

    setNameError(nameError);
    setFileError(fileError);
    if (nameError || fileError) {
      return;
    }

    const bytes = new Uint8Array(await file.arrayBuffer());
    try {
      const component = Graph.addComponent(name, bytes) as Component;
      component.color = selectedColor;
      component.description = description;
      onClose(component);
    } catch (e) {
      setFileError(e.payload);
    }
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setFile(e.dataTransfer.files[0]);
  };

  return (
    <Transition.Root show={show} as={Fragment} afterLeave={handleAfterLeave}>
      <Dialog
        as="div"
        className="relative z-10"
        initialFocus={nameInputRef}
        onClose={() => onClose(null)}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                <form
                  onSubmit={async (e) => {
                    await handleSubmit(e);
                  }}
                >
                  <div className="flex-1 overflow-y-auto">
                    <div className="bg-gray-700 py-6 px-4 sm:px-6">
                      <div className="flex items-center justify-between">
                        <h2 className="text-lg font-medium text-white">
                          Add Component
                        </h2>
                      </div>
                      <div className="mt-1">
                        <p className="text-sm text-gray-300">
                          Add a WebAssembly component to the component library
                        </p>
                      </div>
                    </div>
                    <div className="flex flex-1 flex-col justify-between">
                      <div className="divide-y divide-gray-200 px-4 sm:px-6">
                        <div className="space-y-2 pt-4 pb-5">
                          <div>
                            <label
                              htmlFor="name"
                              className="block text-sm font-medium text-gray-600"
                            >
                              Name
                            </label>
                            <div className="relative mt-1 rounded-md shadow-sm">
                              <input
                                ref={nameInputRef}
                                type="text"
                                name="name"
                                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                aria-invalid="true"
                              ></input>
                              {nameError ? (
                                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                                  <ExclamationCircleIcon
                                    className="h-5 w-5 text-red-500"
                                    aria-hidden="true"
                                  />
                                </div>
                              ) : null}
                            </div>
                            {nameError ? (
                              <p
                                className="mt-2 text-sm text-red-600"
                                id="name-error"
                              >
                                {nameError}
                              </p>
                            ) : null}
                          </div>
                          <div>
                            <div className="flex justify-between">
                              <label
                                htmlFor="description"
                                className="block text-sm font-medium text-gray-600"
                              >
                                Description
                              </label>
                              <span className="text-sm text-gray-400">
                                Optional
                              </span>
                            </div>
                            <div className="relative mt-1 rounded-md shadow-sm">
                              <input
                                ref={descriptionInputRef}
                                type="text"
                                name="description"
                                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                aria-invalid="true"
                              ></input>
                            </div>
                          </div>
                          <div>
                            <Listbox
                              value={selectedColor}
                              onChange={setSelectedColor}
                            >
                              {({ open }) => (
                                <>
                                  <Listbox.Label className="block text-sm font-medium text-gray-600">
                                    Color
                                  </Listbox.Label>
                                  <div className="relative mt-1">
                                    <Listbox.Button className="relative w-full cursor-default rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm">
                                      <div
                                        className={`${selectedColor} h-4`}
                                      ></div>
                                      <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                                        <ChevronUpDownIcon
                                          className="h-5 w-5 text-gray-400"
                                          aria-hidden="true"
                                        />
                                      </span>
                                    </Listbox.Button>

                                    <Transition
                                      show={open}
                                      as={Fragment}
                                      leave="transition ease-in duration-100"
                                      leaveFrom="opacity-100"
                                      leaveTo="opacity-0"
                                    >
                                      <Listbox.Options className="absolute z-10 mt-1 max-h-32 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
                                        {Colors.map((color, i) => (
                                          <Listbox.Option
                                            key={i}
                                            className={({ active }) =>
                                              classNames(
                                                active
                                                  ? "text-white bg-indigo-600"
                                                  : "text-gray-900",
                                                "relative cursor-default select-none py-2 pl-3 pr-9",
                                              )
                                            }
                                            value={color}
                                          >
                                            {({ selected, active }) => (
                                              <>
                                                <div
                                                  className={`${color} h-4`}
                                                ></div>
                                                {selected ? (
                                                  <span
                                                    className={classNames(
                                                      active
                                                        ? "text-white"
                                                        : "text-indigo-600",
                                                      "absolute inset-y-0 right-0 flex items-center pr-4",
                                                    )}
                                                  >
                                                    <CheckIcon
                                                      className="h-5 w-5"
                                                      aria-hidden="true"
                                                    />
                                                  </span>
                                                ) : null}
                                              </>
                                            )}
                                          </Listbox.Option>
                                        ))}
                                      </Listbox.Options>
                                    </Transition>
                                  </div>
                                </>
                              )}
                            </Listbox>
                          </div>
                        </div>
                        <div className="border-none">
                          <div className="flex items-center justify-center w-full">
                            <label
                              htmlFor="file"
                              className="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-white hover:bg-gray-50 focus-within:border-indigo-500 focus-within:ring-indigo-500"
                              onDragOver={handleDragOver}
                              onDrop={handleDrop}
                            >
                              {file ? (
                                <p className="mb-2 text-gray-500 mr-1 ml-1 text-center">
                                  {file
                                    ? file.name
                                    : "Click to select or drop a WebAssembly component here"}
                                </p>
                              ) : (
                                <div className="flex flex-col items-center justify-center pt-5 pb-6">
                                  <p className="mb-2 text-sm text-gray-500 mr-1 ml-1 font-semibold text-center">
                                    {file
                                      ? file.name
                                      : "Click to select or drop a WebAssembly component here"}
                                  </p>
                                  <ChevronDoubleDownIcon className="mt-4 h-8 w-8 text-indigo-400" />
                                </div>
                              )}
                              <input
                                id="file"
                                className="input-file"
                                type="file"
                                accept=".wasm,.wat"
                                onChange={(e) => {
                                  setFile(
                                    (e.target as HTMLInputElement).files[0],
                                  );
                                }}
                              />
                            </label>
                          </div>
                          {fileError ? (
                            <pre className="mt-2 text-sm text-red-600 overflow-x-auto whitespace-pre-wrap">
                              {fileError}
                            </pre>
                          ) : null}
                        </div>
                      </div>
                    </div>
                  </div>
                  <div className="m-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                    <button
                      type="submit"
                      value="submit"
                      className="inline-flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-base font-medium text-white shadow-sm enabled:hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:col-start-2 sm:text-sm disabled:opacity-25"
                    >
                      Add
                    </button>
                    <button
                      type="button"
                      className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:col-start-1 sm:mt-0 sm:text-sm"
                      onClick={() => onClose(null)}
                    >
                      Cancel
                    </button>
                  </div>
                </form>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};

type DownloadComponentDialogProps = {
  show: boolean;
  onClose: (component: Component | null) => void;
  onSubmit: (name: string) => string;
};

export const DownloadComponentDialog = ({
  show,
  onClose,
  onSubmit,
}: DownloadComponentDialogProps) => {
  const { exportedInstance, pushNotification } = useAppState();
  const nameInputRef = useRef<HTMLInputElement>(null);
  const [nameError, setNameError] = useState("");
  const [defineComponents, setDefineComponents] = useState(true);
  const [addComponent, setAddComponent] = useState(true);
  const descriptionInputRef = useRef<HTMLInputElement>(null);
  const [selectedColor, setSelectedColor] = useState(randomColor);

  const handleAfterLeave = () => {
    setNameError("");
    setDefineComponents(true);
    setAddComponent(true);
    setSelectedColor(randomColor());
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    let nameError = "";
    const name = nameInputRef.current?.value ?? "";
    const description = descriptionInputRef?.current.value ?? "";

    if (name.length === 0) {
      nameError = "Name is required";
      nameInputRef.current.focus();
    } else if (addComponent) {
      nameError = onSubmit(name);
    }

    setNameError(nameError);
    if (nameError) {
      return;
    }

    try {
      const bytes = Graph.encodeGraph({
        defineComponents,
        export: exportedInstance?.id,
        validate: true,
      });

      downloadFile(name, bytes);

      let component = null;
      if (addComponent) {
        component = Graph.addComponent(name, bytes);
        component.color = selectedColor;
        component.description = description;
      }

      onClose(component);
    } catch (e) {
      pushNotification({
        type: NotificationType.Error,
        title: "Download Failed",
        message: e.payload,
      });

      onClose(null);
    }
  };

  return (
    <Transition.Root show={show} as={Fragment} afterLeave={handleAfterLeave}>
      <Dialog
        as="div"
        className="relative z-10"
        initialFocus={nameInputRef}
        onClose={() => onClose(null)}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                <form
                  onSubmit={async (e) => {
                    await handleSubmit(e);
                  }}
                >
                  <div className="flex-1 overflow-y-auto">
                    <div className="bg-gray-700 py-6 px-4 sm:px-6">
                      <div className="flex items-center justify-between">
                        <h2 className="text-lg font-medium text-white">
                          Download Component
                        </h2>
                      </div>
                      <div className="mt-1">
                        <p className="text-sm text-gray-300">
                          Download the graph as a new WebAssembly component
                        </p>
                      </div>
                    </div>
                    <div className="flex flex-1 flex-col justify-between">
                      <div className="px-4 sm:px-6">
                        <div className="space-y-2 mt-4 mb-4">
                          <div>
                            <label
                              htmlFor="name"
                              className="block text-sm font-medium text-gray-600"
                            >
                              Name
                            </label>
                            <div className="relative mt-1 rounded-md shadow-sm">
                              <input
                                ref={nameInputRef}
                                type="text"
                                name="name"
                                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                aria-invalid="true"
                              ></input>
                              {nameError ? (
                                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                                  <ExclamationCircleIcon
                                    className="h-5 w-5 text-red-500"
                                    aria-hidden="true"
                                  />
                                </div>
                              ) : null}
                            </div>
                            {nameError ? (
                              <p
                                className="mt-2 text-sm text-red-600"
                                id="name-error"
                              >
                                {nameError}
                              </p>
                            ) : null}
                          </div>
                        </div>
                        <div className="relative flex items-start mb-4">
                          <div className="flex h-5 items-center">
                            <input
                              id="define"
                              checked={defineComponents}
                              onChange={(e) =>
                                setDefineComponents(e.target.checked)
                              }
                              aria-describedby="define-description"
                              name="define"
                              type="checkbox"
                              className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                            />
                          </div>
                          <div className="ml-3 text-sm">
                            <label
                              htmlFor="define"
                              className="font-medium text-gray-700"
                            >
                              Define component dependencies
                            </label>
                            <span
                              id="import-description"
                              className="text-gray-500"
                            >
                              <span className="sr-only">
                                Define component dependencies
                              </span>
                              &nbsp;instead of importing
                            </span>
                          </div>
                        </div>
                        <div className="relative flex items-start mb-4">
                          <div className="flex h-5 items-center">
                            <input
                              id="add"
                              checked={addComponent}
                              onChange={(e) =>
                                setAddComponent(e.target.checked)
                              }
                              aria-describedby="add-description"
                              name="add"
                              type="checkbox"
                              className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                            />
                          </div>
                          <div className="ml-3 text-sm">
                            <label
                              htmlFor="add"
                              className="font-medium text-gray-700"
                            >
                              Add the new component
                            </label>
                            <span
                              id="add-description"
                              className="text-gray-500"
                            >
                              <span className="sr-only">Add the component</span>
                              &nbsp;to the component library
                            </span>
                          </div>
                        </div>
                        <div className="mb-2">
                          <div className="flex justify-between">
                            <label
                              htmlFor="description"
                              className="block text-sm font-medium text-gray-600"
                            >
                              Description
                            </label>
                            <span className="text-sm text-gray-400">
                              Optional
                            </span>
                          </div>
                          <div className="relative mt-1 rounded-md shadow-sm">
                            <input
                              ref={descriptionInputRef}
                              type="text"
                              name="description"
                              disabled={!addComponent}
                              className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm disabled:bg-gray-100 disabled:text-gray-500"
                              aria-invalid="true"
                            ></input>
                          </div>
                        </div>
                        <div className="mb-1">
                          <Listbox
                            value={selectedColor}
                            onChange={setSelectedColor}
                            disabled={!addComponent}
                          >
                            {({ open }) => (
                              <>
                                <Listbox.Label className="block text-sm font-medium text-gray-600">
                                  Color
                                </Listbox.Label>
                                <div className="mt-1">
                                  <Listbox.Button className="relative w-full cursor-default rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm disabled:bg-gray-100">
                                    <div
                                      className={`${selectedColor} h-4`}
                                    ></div>
                                    <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                                      <ChevronUpDownIcon
                                        className="h-5 w-5 text-gray-400"
                                        aria-hidden="true"
                                      />
                                    </span>
                                  </Listbox.Button>

                                  <Transition
                                    show={open}
                                    as={Fragment}
                                    leave="transition ease-in duration-100"
                                    leaveFrom="opacity-100"
                                    leaveTo="opacity-0"
                                  >
                                    <Listbox.Options className="z-10 mt-1 mb-1 max-h-32 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-sm ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
                                      {Colors.map((color, i) => (
                                        <Listbox.Option
                                          key={i}
                                          className={({ active }) =>
                                            classNames(
                                              active
                                                ? "text-white bg-indigo-600"
                                                : "text-gray-900",
                                              "relative cursor-default select-none py-2 pl-3 pr-9",
                                            )
                                          }
                                          value={color}
                                        >
                                          {({ selected, active }) => (
                                            <>
                                              <div
                                                className={`${color} h-4`}
                                              ></div>
                                              {selected ? (
                                                <span
                                                  className={classNames(
                                                    active
                                                      ? "text-white"
                                                      : "text-indigo-600",
                                                    "absolute inset-y-0 right-0 flex items-center pr-4",
                                                  )}
                                                >
                                                  <CheckIcon
                                                    className="h-5 w-5"
                                                    aria-hidden="true"
                                                  />
                                                </span>
                                              ) : null}
                                            </>
                                          )}
                                        </Listbox.Option>
                                      ))}
                                    </Listbox.Options>
                                  </Transition>
                                </div>
                              </>
                            )}
                          </Listbox>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div className="m-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                    <button
                      type="submit"
                      value="submit"
                      className="inline-flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-base font-medium text-white shadow-sm enabled:hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:col-start-2 sm:text-sm disabled:opacity-25"
                    >
                      Download
                    </button>
                    <button
                      type="button"
                      className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:col-start-1 sm:mt-0 sm:text-sm"
                      onClick={() => onClose(null)}
                    >
                      Cancel
                    </button>
                  </div>
                </form>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};

type ConfirmRemoveDialogProps = {
  show: boolean;
  onClose: (show: boolean) => void;
};

export const ConfirmRemoveDialog = ({
  show,
  onClose,
}: ConfirmRemoveDialogProps) => {
  const cancelButtonRef = useRef(null);

  return (
    <Transition.Root show={show} as={Fragment}>
      <Dialog
        as="div"
        className="relative z-10"
        initialFocus={cancelButtonRef}
        onClose={() => onClose(false)}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                <div className="sm:flex sm:items-start">
                  <div className="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                    <ExclamationTriangleIcon
                      className="h-6 w-6 text-red-600"
                      aria-hidden="true"
                    />
                  </div>
                  <div className="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                    <Dialog.Title
                      as="h3"
                      className="text-lg font-medium leading-6 text-gray-900"
                    >
                      Remove Component
                    </Dialog.Title>
                    <div className="mt-2">
                      <p className="text-sm text-gray-500">
                        Are you sure you want to remove this component from the
                        component library?
                      </p>
                    </div>
                  </div>
                </div>
                <div className="mt-5 sm:mt-4 sm:flex sm:flex-row-reverse">
                  <button
                    type="button"
                    className="inline-flex w-full justify-center rounded-md border border-transparent bg-red-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm"
                    onClick={() => onClose(true)}
                  >
                    Remove
                  </button>
                  <button
                    type="button"
                    className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:w-auto sm:text-sm"
                    onClick={() => onClose(false)}
                    ref={cancelButtonRef}
                  >
                    Cancel
                  </button>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};
