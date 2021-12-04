use std::error::Error;

use advent_utils::Part;
use yew::{
    macros::html, ChangeData, Component, ComponentLink, Html, InputData, MouseEvent, ShouldRender,
};

use crate::days::Day;

#[derive(Debug)]
pub(crate) struct App {
    link: ComponentLink<Self>,
    current_part: Part,
    day: Day,
    input_data: String,
    solution: Option<Result<String, Box<dyn Error>>>,
}

#[derive(Debug)]
pub(crate) enum Message {
    ChooseDay(Day),
    ChoosePart(Part),
    UpdateInputData(String),
    Evaluate,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            current_part: Part::One,
            day: Day::Day01,
            solution: None,
            input_data: String::new(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ChooseDay(day) => {
                self.day = day;
                self.current_part = *day
                    .implemented_parts()
                    .first()
                    .expect("day with zero implemented parts");
                self.input_data.clear();
                self.solution = None;
            }
            Message::ChoosePart(part) => {
                self.current_part = part;
                self.solution = None;
            }
            Message::UpdateInputData(data) => self.input_data = data,
            Message::Evaluate => {
                self.solution = Some(self.day.solve(self.current_part, &self.input_data));
            }
        }

        true
    }

    fn view(&self) -> Html {
        let parts = self
            .day
            .implemented_parts()
            .into_iter()
            .enumerate()
            .map(|(idx, part)| {
                let id = format!("part-{}", idx);

                html! {
                    <section>
                        <input
                            checked={self.current_part == part}
                            type="radio"
                            name="part"
                            value={idx.to_string()}
                            id={id.to_string()}
                            onchange=self.link.callback(move |_e| Message::ChoosePart(part))
                        />
                        <label for={id.to_string()}>{format!("Part {:?}", part)}</label>
                    </section>
                }
            });

        let solution = match &self.solution {
            Some(result) => match result {
                Ok(answer) => html! {
                    <p>
                        { "Here is your answer: " }
                        <pre><code> { answer }</code></pre>
                    </p>
                },
                Err(e) => html! {
                    <p>
                        { "There was an error computing your answer: " }
                        <pre><code> { e }</code></pre>
                    </p>
                },
            },
            None => html! {
                <p>
                    { "Press" }
                    <mark>{ "Evaluate" }</mark>
                    { "to get solution" }
                </p>
            },
        };

        html! {
            <main>
                <h1>
                    {"Advent of Code-2020"}
                </h1>
                <h2>
                    {"Results"}
                </h2>
                { solution }
                <form>
                    <h2>
                        {"Select day and part"}
                    </h2>
                    <select
                        onchange=self.link.callback(|e: ChangeData| {
                            if let ChangeData::Select(e) = e {
                                Message::ChooseDay(Day::DAYS[e.selected_index() as usize])
                            } else {
                                unreachable!()
                            }
                        })
                        value={(self.day.day_number() - 1).to_string()}
                    >
                        { for Day::DAYS.iter().map(|day| html! {
                            <option value={(*day as usize).to_string()}>{ day }</option>
                        })}
                    </select>
                    { for parts }
                    <h2>
                        {"Enter input data:"}
                    </h2>
                    <textarea
                        id="input-data"
                        rows="10"
                        cols="80"
                        value={self.input_data.to_owned()}
                        spellcheck="false"
                        oninput=self.link.callback(|e: InputData| Message::UpdateInputData(e.value))
                    />
                    <section>
                        <button onclick=self.link.callback(|e: MouseEvent| {
                            e.prevent_default();
                            Message::Evaluate
                        })>
                            {"Evaluate"}
                        </button>
                    </section>
                </form>
            </main>
        }
    }
}
