use crate::components::control::{Cities, Control};
use crate::components::map_component::{City, MapComponent, Point};
use yew::prelude::*;
mod components;

enum Msg {
    SelectCity(City),
}

struct Model {
    city: City,
    cities: Cities,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let aachen = City {
            name: "Aachen".to_string(),
            lat: Point(50.7597f64, 6.0967f64),
        };
        let stuttgart = City {
            name: "Stuttgart".to_string(),
            lat: Point(48.7784f64, 9.1742f64),
        };
        let cities: Cities = Cities {
            list: vec![aachen, stuttgart],
        };
        let city = cities.list[0].clone();
        Self { city, cities, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectCity(city) => {
                self.city = self
                    .cities
                    .list
                    .iter()
                    .find(|c| c.name == city.name)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(|name| Msg::SelectCity(name));
        html! {
            <>
                <MapComponent city=&self.city  />
                <Control select_city=cb cities=&self.cities/>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    let document = yew::utils::document();
    let app = document.query_selector("#yew").unwrap().unwrap();

    yew::App::<Model>::new().mount(app);

    yew::run_loop();
}
