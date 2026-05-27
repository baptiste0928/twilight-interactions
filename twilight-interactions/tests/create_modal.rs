use twilight_interactions::modal::{CreateModal, ModalData};
use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;

#[derive(CreateModal, Debug, PartialEq, Eq)]
#[modal(title = "Modal Title")]
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

#[test]
fn test_create_modal() {
    let components = vec![
        Component::TextInput(TextInput {
            custom_id: "foo".into(),
            label: "Foo".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: Some(true),
            style: TextInputStyle::Paragraph,
            value: Some("foo_value".into()),
        }),
        Component::TextInput(TextInput {
            custom_id: "bar".into(),
            label: "Label!!".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: Some(false),
            style: TextInputStyle::Short,
            value: None,
        }),
        Component::TextInput(TextInput {
            custom_id: "baz".into(),
            label: "Baz".into(),
            max_length: Some(10),
            min_length: Some(5),
            placeholder: Some("baz_placeholder".into()),
            required: Some(false),
            style: TextInputStyle::Paragraph,
            value: Some("baz_value".into()),
        }),
    ]
    .into_iter()
    .map(|field| {
        Component::ActionRow(ActionRow {
            components: vec![field],
        })
    })
    .collect();

    let expected = ModalData {
        custom_id: "demo_modal".into(),
        title: "Modal Title".into(),
        components,
    };

    assert_eq!(DemoModal::create_modal("demo_modal".into()), expected);
}
