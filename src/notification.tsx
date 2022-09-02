import { Fragment, useEffect, useState } from "react";
import { Transition } from "@headlessui/react";
import { CheckCircleIcon } from "@heroicons/react/24/outline";
import { XCircleIcon, XMarkIcon } from "@heroicons/react/20/solid";
import { NotificationType, useAppState } from "./state";

type NotificationIconProps = {
  type: NotificationType | null;
};

const NotificationIcon = ({ type }: NotificationIconProps) => {
  switch (type) {
    case NotificationType.Success:
      return (
        <CheckCircleIcon
          className="h-6 w-6 text-green-400"
          aria-hidden="true"
        />
      );
    case NotificationType.Error:
      return (
        <XCircleIcon className="h-6 w-6 text-red-400" aria-hidden="true" />
      );
    default:
      return null;
  }
};

const NotificationPopup = () => {
  const { notifications, popNotification } = useAppState();
  const [show, setShow] = useState(false);

  let timer = null;

  useEffect(() => {
    setShow(notifications.length > 0);
  }, [notifications]);

  const handleClose = () => {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }

    setShow(false);

    setTimeout(() => {
      popNotification();
    }, 500);
  };

  useEffect(() => {
    if (show) {
      timer = setTimeout(() => {
        handleClose();
      }, 10000);
    }
  }, [show]);

  const notification = notifications[0];

  return (
    <>
      <div
        aria-live="assertive"
        className="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6 z-50"
      >
        <div className="flex w-full flex-col items-center space-y-4 sm:items-end">
          <Transition
            show={show}
            as={Fragment}
            enter="transform ease-out duration-300 transition"
            enterFrom="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
            enterTo="translate-y-0 opacity-100 sm:translate-x-0"
            leave="transition ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg bg-white shadow-lg ring-1 ring-black ring-opacity-5">
              <div className="p-4">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    <NotificationIcon type={notification?.type} />
                  </div>
                  <div className="ml-3 w-0 flex-1 pt-0.5">
                    <p className="text-sm font-medium text-gray-900">
                      {notification?.title}
                    </p>
                    <p className="mt-1 text-sm text-gray-500">
                      {notification?.message}
                    </p>
                  </div>
                  <div className="ml-4 flex flex-shrink-0">
                    <button
                      type="button"
                      className="inline-flex rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                      onClick={handleClose}
                    >
                      <span className="sr-only">Close</span>
                      <XMarkIcon className="h-5 w-5" aria-hidden="true" />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </Transition>
        </div>
      </div>
    </>
  );
};

export default NotificationPopup;
