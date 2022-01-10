use super::map_component::City;
use yew::services::ConsoleService;
use yew::{html::ImplicitClone, prelude::*};

pub enum Msg {
    CityChosen(City),
}

pub struct Control {
    link: ComponentLink<Self>,
    cities: Vec<City>,
    props: Props,
}

#[derive(Clone)]
pub struct Cities {
    pub list: Vec<City>,
}

impl ImplicitClone for Cities {}

#[derive(Properties, Clone)]
pub struct Props {
    pub cities: Cities,
    pub select_city: Callback<City>,
}

impl Control {
    fn button(&self, city: City) -> Html {
        let name = city.name.clone();
        let cb = self.link.callback(move |_| Msg::CityChosen(city.clone()));
        html! {
            <button onclick=cb>{name}</button>
        }
    }
}

impl Component for Control {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Control {
            link,
            cities: props.cities.list.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CityChosen(city) => {
                ConsoleService::info(format!("Update: {:?}", city.name).as_ref());
                self.props.select_city.emit(city);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="control component-container">
                <h1>{"Choose a city"}</h1>
                <div>
                    {for self.cities.iter().map(|city| Self::button(&self, city.clone()))}
                    </div>

            </div>
        }
    }
}
