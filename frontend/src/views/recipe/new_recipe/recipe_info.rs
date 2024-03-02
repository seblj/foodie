use std::time::Duration;

use crate::{
    components::icons::file_upload_icon::FileUploadIcon,
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
};
use common::recipe::CreateRecipe;
use leptos::*;
use web_sys::{File, Url};

use crate::components::{
    dropdown::DropDownItem,
    form::{
        form_fields::{
            form_field_duration::FormFieldDuration,
            form_field_input::{FormFieldInput, FormFieldInputType},
            form_field_select::FormFieldSelect,
            form_field_textarea::FormFieldTextarea,
        },
        FormGroup,
    },
};

#[component]
pub fn RecipeInfo(file: RwSignal<Option<File>>) -> impl IntoView {
    let items = (0..72)
        .map(|i| DropDownItem {
            key: i,
            value: i,
            label: i.to_string(),
        })
        .collect::<Vec<_>>();

    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"General info about you recipe"</h2>
                <FormGroup>
                    <FileInput file=file/>

                    <FormFieldInput
                        value=move || recipe().name
                        ty=FormFieldInputType::Text
                        placeholder="Name"
                        on_input=move |name| recipe.update(|r| r.name = name)
                    />

                    <FormFieldSelect
                        value=(move || recipe().servings).into_signal()
                        items=items
                        placeholder="Servings"
                        on_change=move |servings| {
                            recipe.update(|r| r.servings = servings.unwrap_or_default())
                        }
                    />

                    <FormFieldDuration
                        max_hours=72
                        placeholder="Baking time"
                        on_change=move |baking_time| {
                            recipe.update(|r| r.baking_time = Some(baking_time))
                        }
                    />

                    <FormFieldDuration
                        max_hours=72
                        placeholder="Prep time"
                        on_change=move |prep_time| {
                            recipe.update(|r| r.prep_time = Some(prep_time))
                        }
                    />

                    <FormFieldTextarea
                        value=move || recipe().description
                        on_input=move |desc| recipe.update(|r| r.description = Some(desc))
                        placeholder="Description"
                    />
                </FormGroup>
            </div>
        </div>
    }
}

#[component]
fn FileInput(file: RwSignal<Option<File>>) -> impl IntoView {
    // TODO: I think I should shrink this maybe to not span 12 columns on desktop.
    // I want to have a square for the photo I think
    let toast = use_toast().unwrap();

    let file_input = create_node_ref::<html::Input>();

    let on_change = move |_| {
        let Some(files) = file_input.get().unwrap().files() else {
            return;
        };

        if files.length() > 1 {
            toast.add(Toast {
                ty: ToastType::Error,
                body: "Only allowed to upload 1 file".to_string(),
                timeout: Some(Duration::from_secs(5)),
            });
            return;
        }

        file.set(files.get(0));
    };

    view! {
        <div class="col-span-12">
            <label
                for="dropzone-file"
                class="flex flex-col items-center justify-center w-full h-64 border-2 rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-gray-600 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500"
            >
                // TODO: See if I can remove some of these tailwind-classes
                {move || {
                    if file().is_some() {
                        view! {
                            <img
                                class="max-h-full"
                                src=move || {
                                    let blob = file().unwrap().slice().unwrap();
                                    Url::create_object_url_with_blob(&blob).unwrap()
                                }
                            />

                            <input
                                accept="image/*"
                                node_ref=file_input
                                id="dropzone-file"
                                type="file"
                                on:change=on_change
                                class="hidden"
                            />
                        }
                            .into_view()
                    } else {
                        view! {
                            // TODO: Maybe not unwrap on slice?

                            <div class="flex flex-col items-center justify-center pt-5 pb-6">
                                <FileUploadIcon/>
                                <p class="mb-2 text-sm text-gray-500 dark:text-gray-400 font-semibold">
                                    "Upload image for your recipe"
                                </p>
                            </div>
                            <input
                                accept="image/*"
                                node_ref=file_input
                                id="dropzone-file"
                                type="file"
                                on:change=on_change
                                class="hidden"
                            />
                        }
                            .into_view()
                    }
                }}

            </label>
        </div>
    }
}
