use twilight_interactions::modal::{ModalInputData, ModalModel};
use twilight_model::application::interaction::modal::{
    ModalInteractionDataActionRow, ModalInteractionDataComponent,
};
use twilight_model::channel::message::component::ComponentType;

#[derive(ModalModel, Debug, PartialEq, Eq)]
struct DemoModal {
    #[modal(
        custom_id = "foo",
        style = "paragraph",
        label = "Foo",
        value = "foo_value"
    )]
    paragraph_field: String,
    #[modal(custom_id = "bar", label = "Label!!", style = "short")]
    short_field_optional: Option<String>,
    #[modal(
        custom_id = "baz",
        style = "paragraph",
        label = "Baz",
        value = "baz_value",
        placeholder = "baz_placeholder",
        min_length = 5,
        max_length = 10
    )]
    paragraph_field_optional: Option<String>,
}

#[derive(ModalModel, Debug, PartialEq, Eq)]
struct UnitModal;

#[test]
fn test_modal_model() {
    let paragraph_field_component = ModalInteractionDataComponent {
        custom_id: "foo".into(),
        kind: ComponentType::TextInput,
        value: Some("ParagraphField".into()),
    };

    let short_field_optional_component = ModalInteractionDataComponent {
        custom_id: "bar".into(),
        kind: ComponentType::TextInput,
        value: Some("ShortFieldOptional".into()),
    };

    let paragraph_field_optional_component = ModalInteractionDataComponent {
        custom_id: "baz".into(),
        kind: ComponentType::TextInput,
        value: None,
    };

    let components = vec![
        ModalInteractionDataActionRow {
            components: vec![paragraph_field_component],
        },
        ModalInteractionDataActionRow {
            components: vec![short_field_optional_component],
        },
        ModalInteractionDataActionRow {
            components: vec![paragraph_field_optional_component],
        },
    ];

    let data = ModalInputData { components };

    let result = DemoModal::from_interaction(data).unwrap();

    assert_eq!(
        DemoModal {
            paragraph_field: "ParagraphField".into(),
            short_field_optional: Some("ShortFieldOptional".into()),
            paragraph_field_optional: None,
        },
        result
    );
}

#[test]
fn test_unit_modal_model() {
    let data = ModalInputData { components: vec![] };

    let result = UnitModal::from_interaction(data).unwrap();

    assert_eq!(UnitModal, result);
}
