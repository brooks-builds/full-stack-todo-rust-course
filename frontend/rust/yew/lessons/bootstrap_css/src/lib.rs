mod text_form_field;

use text_form_field::TextFormField;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <div class="row">
                <div class="col">
                    <h1 class="text-center">{"New Task"}</h1>
                </div>
            </div>
            <section class="row">
                <form>
                    <div class="col">
                        <TextFormField
                            id="task-title"
                            label="Title"
                            placeholder="Pick up groceries"
                            description="Create a new task title" />
                    </div>
                    <div class="col">
                        <label for="task-description" class="form-label">{"Description"}</label>
                        <textarea class="form-control" id="task-description" rows="3" aria-describedby="task-description-help"></textarea>
                        <span class="input-group-text visually-hidden" id="task-description-help">{"Enter task description"}</span>
                    </div>
                </form>
            </section>
        </main>
    }
}
