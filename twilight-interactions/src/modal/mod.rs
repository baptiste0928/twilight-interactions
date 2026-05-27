mod create_modal;
mod modal_model;

pub use create_modal::{CreateModal, ModalData};
pub use modal_model::{ModalComponent, ModalInputData, ModalModel};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use twilight_interactions_derive::{CreateModal, ModalModel};
