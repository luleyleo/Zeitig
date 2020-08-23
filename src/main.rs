use druid::{AppLauncher, WindowDesc};

mod controller;
mod state;
mod ui;
mod widgets;

mod state2;

fn main() {
    let window = WindowDesc::new(ui::tracker)
        .title("Zeitig")
        .window_size((300.0, 400.0));

    let state = state::files::read_state();

    let mut new_state =
        state2::backend::Sqlite::new("./zeitig.db").expect("Failed to create Sqlite backend.");
    new_state.setup().expect("Failed to setup new database.");
    use state2::backend::Backend;
    let mut new_content = state2::Content::default();

    for action in &state.actions {
        let a2 = new_state.create_action(action.as_ref()).unwrap();
        new_content.actions.push_back(a2);
    }

    for subject in &state.subjects {
        let s2 = new_state.create_subject(subject.as_ref()).unwrap();
        new_content.subjects.push_back(s2);
    }

    for ((action, subject), time) in state.time_table.0.iter() {
        let a2 = new_content
            .actions
            .iter()
            .find(|a| a.name.as_ref() == action.as_ref())
            .cloned()
            .unwrap();
        let s2 = new_content
            .subjects
            .iter()
            .find(|s| s.name.as_ref() == subject.as_ref())
            .cloned()
            .unwrap();
        new_state
            .update_time(
                &state2::Topic {
                    action: a2,
                    subject: s2,
                },
                time,
            )
            .unwrap();
    }

    for session in &state.history {
        let action = &session.action;
        let subject = &session.subject;
        let a2 = new_content
            .actions
            .iter()
            .find(|a| a.name.as_ref() == action.as_ref())
            .cloned()
            .unwrap();
        let s2 = new_content
            .subjects
            .iter()
            .find(|s| s.name.as_ref() == subject.as_ref())
            .cloned()
            .unwrap();

        new_state
            .add_session(&state2::Session {
                topic: state2::Topic {
                    action: a2,
                    subject: s2,
                },
                started: session.started.clone(),
                ended: session.ended.clone(),
            })
            .unwrap();
    }

    new_state.close().expect("Failed to close sqlite backend.");

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
        .expect("Failed to launch Zeitig.");
}
