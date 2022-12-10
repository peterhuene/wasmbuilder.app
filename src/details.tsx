import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import Highlight, { defaultProps, Language, Prism } from "prism-react-renderer";
import vsDark from "prism-react-renderer/themes/vsDark";
import { Fragment, useEffect, useState } from "react";
import { ConfirmRemoveDialog } from "./dialogs";
import { Component, NotificationType, useAppState } from "./state";

Prism.languages["wit"] = {
  comment: {
    pattern: /\/\/.*|\/\*[\s\S]*?(?:\*\/|$)/,
    greedy: true,
  },
  operator: {
    pattern: /=|,|:|;|\(|\)|\{|\}|<|>|\*|->/,
  },
  keyword: {
    pattern:
      /(^|\s)(use|type|func|u8|u16|u32|u64|s8|s16|s32|s64|float32|float64|char|record|flags|variant|enum|union|bool|string|option|result|future|stream|list|_|as|from|static|interface|tuple|implements|import|export|world|default)\b/,
  },
  identifier: {
    // TODO: support unicode identifiers
    pattern: /\b%?[a-z][a-z0-9-]*\b/u,
    alias: "symbol",
  },
};

type ComponentDetailsProps = {
  onRemoveComponent: (component: Component) => void;
};

export const ComponentDetails = ({
  onRemoveComponent,
}: ComponentDetailsProps) => {
  const { selectedComponent, selectComponent, pushNotification } =
    useAppState();
  const [showDetails, setShowDetails] = useState(false);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);

  useEffect(() => {
    setShowDetails(selectedComponent != null);
  }, [selectedComponent]);

  const selectedName = selectedComponent?.name ?? "";
  const selectedDescription = selectedComponent?.description ?? "";
  const selectedImports = selectedComponent?.imports ?? [];
  const selectedExports = selectedComponent?.exports ?? [];
  const selectedWit = selectedComponent?.wit ?? "";

  const handleConfirmRemoveComponent = (remove: boolean) => {
    setShowConfirmDialog(false);
    if (remove) {
      setShowDetails(false);
      onRemoveComponent(selectedComponent);
      pushNotification({
        type: NotificationType.Success,
        title: "Component Removed",
        message: `Component '${selectedName}' has been removed.`,
      });
    }
  };

  const handleAfterLeave = () => {
    selectComponent(null);
  };

  return (
    <Transition.Root
      show={showDetails}
      as={Fragment}
      afterLeave={handleAfterLeave}
    >
      <Dialog
        as="div"
        className="relative z-10"
        onClose={() => setShowDetails(false)}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-in-out duration-500"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in-out duration-500"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-hidden">
          <div className="absolute inset-0 overflow-hidden">
            <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full">
              <Transition.Child
                as={Fragment}
                enter="transform transition ease-in-out duration-500 sm:duration-700"
                enterFrom="translate-x-full"
                enterTo="translate-x-0"
                leave="transform transition ease-in-out duration-500 sm:duration-700"
                leaveFrom="translate-x-0"
                leaveTo="translate-x-full"
              >
                <Dialog.Panel className="pointer-events-auto w-screen max-w-xl">
                  <div className="flex h-full flex-col overflow-y-auto bg-white shadow-xl">
                    <div className="flex-1">
                      <div className="bg-gray-700 px-4 py-6">
                        <div className="flex items-start justify-between space-x-3">
                          <div className="space-y-1">
                            <Dialog.Title className="text-lg font-medium text-white">
                              {selectedName}
                            </Dialog.Title>
                            <p className="text-md text-gray-300">
                              {selectedDescription}
                            </p>
                          </div>
                          <div className="flex h-7 items-center">
                            <button
                              type="button"
                              className="text-gray-400 hover:text-gray-500"
                              onClick={() => setShowDetails(false)}
                            >
                              <span className="sr-only">Close panel</span>
                              <XMarkIcon
                                className="h-6 w-6"
                                aria-hidden="true"
                              />
                            </button>
                          </div>
                        </div>
                      </div>

                      <div className="px-4 mt-4">
                        <div>
                          <h1 className="text-xl font-semibold text-gray-900">
                            Imports
                          </h1>
                        </div>
                        {selectedImports.length > 0 ? (
                          <div className="mt-4 min-w-full max-h-80 overflow-y-auto align-middle shadow ring-1 ring-black ring-opacity-5 rounded-lg">
                            <table className="min-w-full divide-y divide-gray-300">
                              <thead className="bg-gray-50">
                                <tr>
                                  <th
                                    scope="col"
                                    className="whitespace-nowrap px-2 py-3.5 text-left text-sm font-semibold text-gray-900"
                                  >
                                    Name
                                  </th>
                                  <th
                                    scope="col"
                                    className="whitespace-nowrap px-2 py-3.5 text-left text-sm font-semibold text-gray-900"
                                  >
                                    Kind
                                  </th>
                                </tr>
                              </thead>
                              <tbody className="divide-y divide-gray-200 bg-white">
                                {selectedImports.map(({ name, kind }) => (
                                  <tr key={name}>
                                    <td className="whitespace-nowrap px-2 py-2 text-sm text-gray-900">
                                      {name}
                                    </td>
                                    <td className="whitespace-nowrap px-2 py-2 text-sm text-gray-900">
                                      {kind}
                                    </td>
                                  </tr>
                                ))}
                              </tbody>
                            </table>
                          </div>
                        ) : (
                          <div className="py-2 text-sm font-medium text-gray-600">
                            The component has no imports
                          </div>
                        )}
                      </div>

                      <div className="px-4 mt-4">
                        <div>
                          <h1 className="text-xl font-semibold text-gray-900">
                            Exports
                          </h1>
                        </div>
                        {selectedExports.length > 0 ? (
                          <div className="mt-4 min-w-full max-h-80 overflow-y-auto align-middle shadow ring-1 ring-black ring-opacity-5 rounded-lg">
                            <table className="min-w-full divide-y divide-gray-300">
                              <thead className="bg-gray-50">
                                <tr>
                                  <th
                                    scope="col"
                                    className="whitespace-nowrap px-2 py-3.5 text-left text-sm font-semibold text-gray-900"
                                  >
                                    Name
                                  </th>
                                  <th
                                    scope="col"
                                    className="whitespace-nowrap px-2 py-3.5 text-left text-sm font-semibold text-gray-900"
                                  >
                                    Kind
                                  </th>
                                </tr>
                              </thead>
                              <tbody className="divide-y divide-gray-200 bg-white">
                                {selectedExports.map(({ name, kind }) => (
                                  <tr key={name}>
                                    <td className="whitespace-nowrap px-2 py-2 text-sm text-gray-900">
                                      {name}
                                    </td>
                                    <td className="whitespace-nowrap px-2 py-2 text-sm text-gray-900">
                                      {kind}
                                    </td>
                                  </tr>
                                ))}
                              </tbody>
                            </table>
                          </div>
                        ) : (
                          <div className="py-2 text-sm font-medium text-gray-600">
                            The component has no exports
                          </div>
                        )}
                      </div>

                      <div className="p-4">
                        <div>
                          <h1 className="text-xl font-semibold text-gray-900">
                            Interface
                          </h1>
                        </div>
                        {selectedWit.length > 0 ? (
                          <div className="mt-2 max-h-96 overflow-y-auto">
                            <Highlight
                              {...defaultProps}
                              code={selectedWit}
                              theme={vsDark}
                              language={"wit" as Language}
                            >
                              {({
                                className,
                                style,
                                tokens,
                                getLineProps,
                                getTokenProps,
                              }) => (
                                <pre
                                  className={
                                    className + " overflow-x-auto px-4 py-2"
                                  }
                                  style={style}
                                >
                                  {tokens.map((line, i) => (
                                    <div
                                      key={i}
                                      {...getLineProps({ line, key: i })}
                                    >
                                      {line.map((token, key) => (
                                        <span
                                          key={key}
                                          {...getTokenProps({ token, key })}
                                        />
                                      ))}
                                    </div>
                                  ))}
                                </pre>
                              )}
                            </Highlight>
                          </div>
                        ) : (
                          <div className="mt-2 text-sm font-medium text-gray-600">
                            The component has no exported interface
                          </div>
                        )}
                      </div>
                    </div>

                    <ConfirmRemoveDialog
                      show={showConfirmDialog}
                      onClose={handleConfirmRemoveComponent}
                    />

                    <div className="flex-shrink-0 border-t border-gray-200 px-4 py-5">
                      <div className="flex justify-end space-x-3">
                        <button
                          type="button"
                          onClick={() => setShowConfirmDialog(true)}
                          className="inline-flex justify-center rounded-md border border-transparent bg-red-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
                        >
                          Remove Component
                        </button>
                      </div>
                    </div>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};
