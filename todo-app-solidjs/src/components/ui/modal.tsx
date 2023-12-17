import { type Ref } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./typography";
import { classes } from "../../core/utils/classes";

export type ModalRef = { show: () => void; close: () => void };

export type ModalProps = {
  id: string;
  title: string;
  class?: string;
  ref?: Ref<ModalRef>;
  children?: JSX.Element;
};

export function Modal(props: ModalProps): JSX.Element {
  const dialogStyles = (): string => {
    return classes("modal").add(props.class).build();
  };

  const bindDialogRef = (dialog: HTMLDialogElement): void => {
    if (typeof props.ref === "function") {
      props.ref({
        close: () => dialog.close(),
        show: () => dialog.showModal(),
      });
    }
  };

  return (
    <dialog ref={bindDialogRef} id={props.id} class={dialogStyles()}>
      <div class="modal-box">
        <Typography.Title level={3}>{props.title}</Typography.Title>

        {props.children}
      </div>

      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
  );
}
