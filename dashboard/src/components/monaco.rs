use leptos::{html::Div, prelude::*};
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
};
use reactive_stores::Store;
use std::{cell::RefCell, option, rc::Rc, sync::Arc};

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    editor: Rc<Option<CodeEditorModel>>,
}

#[component]
pub fn CodeEditor() -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();
    let editor: Rc<RefCell<Option<CodeEditorModel>>> = Default::default();

    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let options = CodeEditorOptions::default()
                .with_language("rust".to_owned())
                .with_value("CONTENT".to_owned())
                .with_builtin_theme(BuiltinTheme::VsDark)
                .with_automatic_layout(true);
            let code_editor = CodeEditorModel::create(element.as_ref(), Some(options));
            code_editor.set_model(
                &TextModel::create("The home index on a text editor", Some("liquid"), None)
                    .unwrap(),
            );
            *editor.borrow_mut() = Some(code_editor);
        }
    });

    view! {
        <div data-editor node_ref=node_ref />
    }
}
