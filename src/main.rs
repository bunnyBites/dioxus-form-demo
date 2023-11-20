#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

fn main() {
    // for logging in browser
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(App);
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    single_option: String,
    multi_option: Vec<String>
}

fn App(cx: Scope) -> Element {
    let onsubmit = move |event: Event<FormData>| {

        match event.parsed_values::<User>() {
            Ok(values) => log::info!("{:?}", values),
            Err(e) => log::info!("{}", e)
        };
    };

    render!(
        div {
            class: "card w-50 mx-auto mt-5",
            div {
                class: "card-body",
                form {
                    onsubmit: onsubmit,

                    // input field
                    div {
                        class: "mb-2",
                        label {
                            class: "form-label",
                            r#for: "username",
                            "Username"
                        }
                        input {
                            id: "username",
                            r#type: "text",
                            class: "form-control",
                            name: "username",
                        },
                    }

                    // single select field
                    div {
                        class: "mb-2",
                        label {
                            class: "form-label",
                            r#for: "single_option",
                            "Select single option"
                        }
                        select {
                            id: "single_option",
                            name: "single_option",
                            class: "form-select mb-2",
                            option {
                                value: "option_one",
                                "Option 1",
                            },
                            option {
                                value: "option_two",
                                "Option 2"
                            }
                        },
                    }

                    // multi select dropdown field
                    div {
                        label {
                            class: "form-label mt-3",
                            r#for: "multi_option",
                            "Select Multiple options"
                        },
                        select {
                            id: "multi_select_option",
                            name: "multi_option",
                            multiple: "true",
                            class: "form-select mt-2",
                            option {
                                value: "multi_option_one",
                                "Option 1"
                            },
                            option {
                                value: "multi_option_two",
                                "Option 2"
                            }
                        }
                    }

                    //submit button
                    button {
                        class: "btn btn-primary w-100 my-4",
                        r#type: "submit",
                        "Submit"
                    }
                }
            }
        }
    )
}