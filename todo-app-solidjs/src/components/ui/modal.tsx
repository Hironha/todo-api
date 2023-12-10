import { onMount, type Ref } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./typography";

export type ModalRef = { show: () => void; close: () => void };

export type ModalProps = {
  id: string;
  title: string;
  ref?: Ref<ModalRef>;
  children?: JSX.Element;
};

export function Modal(props: ModalProps): JSX.Element {
  onMount(() => {
    const dialogRef = document.getElementById(props.id) as HTMLDialogElement | undefined;
    if (dialogRef && typeof props.ref === "function") {
      props.ref({ close: () => dialogRef.close(), show: () => dialogRef.showModal() });
    }
  });

  return (
    <dialog id={props.id} class="modal">
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
