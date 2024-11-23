use gloo_net::http::Request;
use wasm_logger;
use log;

use yew::prelude::*;

use common::distro::Distro;

#[derive(Properties, PartialEq)]
struct DistrosListProps {
    distros: Vec<Distro>,
    on_click: Callback<Distro>
}

#[function_component(DistrosList)]
fn videos_list(
    DistrosListProps { 
        distros,
        on_click,
    }: &DistrosListProps
) -> Html {
    let on_click = on_click.clone();
    distros.iter().map(|distro| {
        let on_video_select = {
            let on_click = on_click.clone();
            let distro = distro.clone();
            Callback::from(move |_| {
                on_click.emit(distro.clone())
            })
        };

        html! {
            <p key={distro.id} onclick={on_video_select}>{format!("{}: {}", distro.url, distro.name_or_path)}</p>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
struct DistrosDetailsProps {
    distro: Distro,
}

#[function_component(DistroDetails)]
fn video_details(DistrosDetailsProps { distro }: &DistrosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ distro.name_or_path.clone() }</h3>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected_distro = use_state(|| None);

    let on_distro_select = {
        let selected_distro = selected_distro.clone();
        Callback::from(move |distro: Distro| {
            selected_distro.set(Some(distro))
        })
    };

    let details = selected_distro.as_ref().map(|distro| html! {
        <DistroDetails distro={distro.clone()} />
    });

    let distros = use_state(|| vec![]);
    {
        let distros = distros.clone();
        use_effect_with((), move |_| {
            let distros = distros.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8080/")
                    .send()
                    .await;
                
                let response = match response {
                    Ok(response) => response,
                    Err(e) => {
                        log::error!("Requesting distros failed: {}", e);
                        return;
                    },
                };

                let fetched_distros: Vec<Distro> = match response.json().await {
                    Ok(distros) => distros,
                    Err(e) => {
                        log::error!("Parsing json response failed: {}", e);
                        return;
                    }
                };

                log::info!("Distros: {:?}", fetched_distros);

                distros.set(fetched_distros);
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "AptBrowsr" }</h1>
            <div>
            <DistrosList distros={(*distros).clone()} on_click={on_distro_select.clone()} />
            </div>
            <div>
                { for details }
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
